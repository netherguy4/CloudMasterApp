use crate::utils::{InstanceInfo, extract_instance_info};
use anyhow::{Error, Result, anyhow};
use google_cloud_compute_v1::client::Instances;
use google_cloud_compute_v1::model::Instance;
use google_cloud_gax::paginator::ItemPaginator;

use once_cell::sync::OnceCell;

static INSTANCE_CLIENT: OnceCell<Instances> = OnceCell::new();

async fn get_instance_client() -> Result<&'static Instances, Error> {
    if INSTANCE_CLIENT.get().is_none() {
        let client = Instances::builder().build().await?;
        INSTANCE_CLIENT.set(client).ok();
    }
    Ok(INSTANCE_CLIENT.get().unwrap())
}

pub async fn fetch_instance(
    project_id: &str,
    instance: &str,
    zone: &str,
) -> Result<Instance, Error> {
    let client = get_instance_client().await?;

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

#[tauri::command]
pub async fn instances_list(project_id: &str) -> Result<Vec<InstanceInfo>, String> {
    let client = get_instance_client()
        .await
        .map_err(|e| format!("Client error: {e}"))?;

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
    let client = get_instance_client()
        .await
        .map_err(|e| format!("Client error: {e}"))?;

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
    let client = get_instance_client()
        .await
        .map_err(|e| format!("Client error: {e}"))?;

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
