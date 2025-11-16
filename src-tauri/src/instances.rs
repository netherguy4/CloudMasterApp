use crate::utils::{InstanceInfo, extract_instance_info};

use anyhow::{Error, Result, anyhow};
use google_cloud_compute_v1::client::Instances;
use google_cloud_compute_v1::model::Instance;
use google_cloud_gax::paginator::ItemPaginator;
use once_cell::sync::OnceCell;
use rayon::prelude::*;

static INSTANCE_CLIENT: OnceCell<Instances> = OnceCell::new();

async fn get_instance_client() -> Result<&'static Instances, Error> {
    if INSTANCE_CLIENT.get().is_none() {
        let client = Instances::builder().build().await?;
        INSTANCE_CLIENT.set(client).ok();
    }
    Ok(INSTANCE_CLIENT.get().unwrap())
}

pub async fn fetch_instance(project_id: &str, instance: &str, zone: &str) -> Result<Instance> {
    let client = get_instance_client().await?;

    client
        .get()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
        .map_err(|e| anyhow!("Error: {}", e))
}

#[tauri::command]
pub async fn instances_list(project_id: &str) -> Result<Vec<InstanceInfo>, String> {
    let client = get_instance_client()
        .await
        .map_err(|e| format!("Client error: {e}"))?;

    let mut paginator = client
        .aggregated_list()
        .set_return_partial_success(true)
        .set_project(project_id)
        .by_item();

    let mut zone_results = Vec::new();

    while let Some(result) = paginator.next().await {
        let (zone, scoped) = result.map_err(|e| format!("Pagination error: {e}"))?;

        if !scoped.instances.is_empty() {
            let zone_instances: Result<Vec<InstanceInfo>, String> = scoped
                .instances
                .into_par_iter()
                .map(|inst| {
                    extract_instance_info(&inst, &zone)
                        .map_err(|e| format!("Failed to process instance in {}: {}", zone, e))
                })
                .collect();

            zone_results.push(zone_instances);
        }
    }

    if zone_results.is_empty() {
        return Err(format!("No instances found for project '{}'", project_id));
    }

    // Now, just flatten the results
    let mut all = Vec::new();
    for zone in zone_results {
        let mut items = zone?;
        all.append(&mut items);
    }

    Ok(all)
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

    extract_instance_info(&instance_obj, zone)
        .map_err(|e| format!("Failed to process instance '{}': {e}", instance))
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

    client
        .start()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
        .map_err(|e| format!("Failed to start instance '{}': {}", instance, e))?;

    Ok(format!(
        "Instance '{}' start operation initiated successfully",
        instance
    ))
}

#[tauri::command]
pub async fn instance_stop(project_id: &str, instance: &str, zone: &str) -> Result<String, String> {
    let client = get_instance_client()
        .await
        .map_err(|e| format!("Client error: {e}"))?;

    client
        .stop()
        .set_instance(instance)
        .set_project(project_id)
        .set_zone(zone)
        .send()
        .await
        .map_err(|e| format!("Failed to stop instance '{}': {}", instance, e))?;

    Ok(format!(
        "Instance '{}' stop operation initiated successfully",
        instance
    ))
}
