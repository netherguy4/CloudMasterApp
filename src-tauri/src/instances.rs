use crate::utils::{JsonResponse, error_response, extract_instance_info, success_response};
use anyhow::{Context, Result};
use google_cloud_compute_v1::client::Instances;
use google_cloud_compute_v1::model::Instance;
use google_cloud_gax::paginator::ItemPaginator;
use log::error;
use serde_json::{Value, json};

pub async fn build_instances_client() -> Result<Instances> {
    let client = Instances::builder()
        .build()
        .await
        .context("Failed to build Instances client")?;
    Ok(client)
}

pub async fn fetch_instance(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<Option<Instance>> {
    let client = build_instances_client().await?;

    match client
        .get()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
    {
        Ok(instance) => Ok(Some(instance)),
        Err(err) => {
            error!("Error retrieving instance {}: {:?}", instance, err);
            Ok(None)
        }
    }
}

pub async fn instance_status(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<Option<String>> {
    if let Some(inst) = fetch_instance(project_id, instance, zone).await? {
        let status = format!("{:?}", inst.status.unwrap_or_default()).to_lowercase();
        Ok(Some(status))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn instances_list(project_id: &str) -> Result<JsonResponse<Value>, String> {
    let client = build_instances_client().await.map_err(|e| e.to_string())?;
    let mut instances_json = Vec::new();

    let mut instances = client
        .aggregated_list()
        .set_return_partial_success(true)
        .set_project(project_id)
        .by_item();

    while let Some(result) = instances.next().await {
        match result {
            Ok((zone, scoped_list)) => {
                if scoped_list.instances.is_empty() {
                    continue;
                }

                for instance in scoped_list.instances {
                    match extract_instance_info(&instance, &zone) {
                        Ok(info) => instances_json.push(info),
                        Err(err) => {
                            error!("Failed to process instance in zone {}: {:?}", zone, err);
                        }
                    }
                }
            }
            Err(err) => {
                error!("Error retrieving instance list: {:?}", err);
            }
        }
    }

    if instances_json.is_empty() {
        Ok(error_response(404, "No instances found"))
    } else {
        Ok(success_response(json!(instances_json)))
    }
}

#[tauri::command]
pub async fn instance_get(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<JsonResponse<Value>, String> {
    let mut instances_json = Vec::new();

    // Try fetching the instance
    let instance_obj = fetch_instance(project_id, instance, zone)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(instance_obj) = instance_obj {
        match extract_instance_info(&instance_obj, zone) {
            Ok(info) => instances_json.push(info),
            Err(err) => {
                error!("Failed to process instance: {:?}", err);
                return Ok(error_response(500, "Failed to process instance"));
            }
        }
    }

    if instances_json.is_empty() {
        Ok(error_response(
            404,
            &format!("Instance '{}' not found", instance),
        ))
    } else {
        Ok(success_response(json!(instances_json)))
    }
}

#[tauri::command]
pub async fn instance_start(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<JsonResponse<Value>, String> {
    let client = build_instances_client().await.map_err(|e| e.to_string())?;

    // Get current instance status
    let status = match instance_status(project_id, instance, zone)
        .await
        .map_err(|e| e.to_string())?
    {
        Some(s) => s,
        None => {
            return Ok(error_response(
                404,
                &format!("Instance '{}' not found", instance),
            ));
        }
    };

    // Prevent starting if not in TERMINATED state
    if status.to_lowercase() != "terminated" {
        return Ok(error_response(
            406,
            &format!("Cannot start '{}': instance is {}", instance, status),
        ));
    }

    // Attempt to start the instance
    match client
        .start()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
    {
        Ok(_) => Ok(success_response(json!(format!(
            "Instance '{}' start operation initiated",
            instance
        )))),
        Err(err) => {
            error!("Failed to start instance '{}': {:?}", instance, err);
            Ok(error_response(
                500,
                &format!("Failed to start instance '{}': {}", instance, err),
            ))
        }
    }
}

#[tauri::command]
pub async fn instance_stop(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<JsonResponse<Value>, String> {
    let client = build_instances_client().await.map_err(|e| e.to_string())?;

    // Get current instance status
    let status = match instance_status(project_id, instance, zone)
        .await
        .map_err(|e| e.to_string())?
    {
        Some(s) => s,
        None => {
            return Ok(error_response(
                404,
                &format!("Instance '{}' not found", instance),
            ));
        }
    };

    // Only stop if running
    if status.to_lowercase() != "running" {
        return Ok(error_response(
            406,
            &format!("Cannot stop '{}': instance is {}", instance, status),
        ));
    }

    // Try stopping the instance
    match client
        .stop()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
    {
        Ok(_) => Ok(success_response(json!(format!(
            "Instance '{}' stop operation initiated",
            instance
        )))),
        Err(err) => {
            error!("Failed to stop instance '{}': {:?}", instance, err);
            Ok(error_response(
                500,
                &format!("Failed to stop instance '{}': {}", instance, err),
            ))
        }
    }
}
