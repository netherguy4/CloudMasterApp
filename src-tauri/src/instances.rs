use crate::utils::{InstanceInfo, extract_instance_info};
use anyhow::{Context, Error, Result, anyhow};
use google_cloud_compute_v1::client::Instances;
use google_cloud_compute_v1::model::Instance;
use google_cloud_gax::paginator::ItemPaginator;

pub async fn build_instances_client() -> Result<Instances, Error> {
    Instances::builder()
        .build()
        .await
        .context("Failed to build Instances client")
}

pub async fn fetch_instance(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<Instance, Error> {
    let client = build_instances_client()
        .await
        .map_err(|e| anyhow!("Error: {}", e))?;

    let instance_obj = client
        .get()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await;

    match instance_obj {
        Ok(instance_obj) => Ok(instance_obj),
        Err(e) => Err(anyhow!("Error: {}", e)),
    }
}

pub async fn instance_status(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<String, Error> {
    let inst = fetch_instance(project_id, instance, zone).await;
    match inst {
        Ok(inst) => inst
            .status
            .as_ref()
            .map(|s| format!("{:?}", s).to_lowercase())
            .ok_or_else(|| anyhow!("Instance '{}' has no status", instance)),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

#[tauri::command]
pub async fn instances_list(project_id: &str) -> Result<Vec<InstanceInfo>, String> {
    let client = build_instances_client()
        .await
        .map_err(|e| format!("Error: {e}"))?;

    let mut instances_array = Vec::new();

    let mut instances = client
        .aggregated_list()
        .set_return_partial_success(true)
        .set_project(project_id)
        .by_item();

    while let Some(result) = instances.next().await {
        let (zone, scoped_list) =
            result.map_err(|e| format!("Error retrieving instance list: {e}"))?;

        if scoped_list.instances.is_empty() {
            continue;
        }

        for instance in scoped_list.instances {
            let info = extract_instance_info(&instance, &zone)
                .map_err(|e| format!("Failed to process instance in zone {}: {e}", zone))?;
            instances_array.push(info);
        }
    }

    if instances_array.is_empty() {
        Err(format!("No instances found for project '{}'", project_id))
    } else {
        Ok(instances_array)
    }
}

#[tauri::command]
pub async fn instance_get(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<InstanceInfo, String> {
    let instance_obj = fetch_instance(project_id, instance, zone)
        .await
        .map_err(|e| format!("Failed to fetch instance '{}': {e}", instance))?;

    let info = extract_instance_info(&instance_obj, zone)
        .map_err(|e| format!("Failed to process instance '{}': {e}", instance))?;

    Ok(info)
}

#[tauri::command]
pub async fn instance_start(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<String, String> {
    let client = build_instances_client()
        .await
        .map_err(|e| format!("Error: {}", e))?;

    let status = match instance_status(project_id, instance, zone).await {
        Ok(status) => status,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };

    if status.trim().is_empty() {
        return Err(format!(
            "Instance '{}' has no valid status (response may be malformed)",
            instance
        ));
    }

    if status.to_lowercase() != "terminated" {
        return Err(format!(
            "Cannot start '{}': instance is currently in '{}' state",
            instance, status
        ));
    }

    let result = client
        .start()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await;

    match result {
        Ok(_) => Ok(format!(
            "Instance '{}' start operation initiated successfully",
            instance
        )),
        Err(e) => Err(format!("Failed to start instance '{}': {}", instance, e)),
    }
}

#[tauri::command]
pub async fn instance_stop(project_id: &str, instance: &str, zone: &str) -> Result<String, String> {
    let client = build_instances_client()
        .await
        .map_err(|e| format!("Error: {}", e))?;

    let status = match instance_status(project_id, instance, zone).await {
        Ok(status) => status,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };

    if status.trim().is_empty() {
        return Err(format!(
            "Instance '{}' has no valid status (response may be malformed)",
            instance
        ));
    }

    if status.to_lowercase() != "running" {
        return Err(format!(
            "Cannot stop '{}': instance is currently in '{}' state",
            instance, status
        ));
    }

    let result = client
        .stop()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await;

    match result {
        Ok(_) => Ok(format!(
            "Instance '{}' stop operation initiated successfully",
            instance
        )),
        Err(e) => Err(format!("Failed to stop instance '{}': {}", instance, e)),
    }
}
