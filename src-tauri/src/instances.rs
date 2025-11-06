use crate::utils::{InstanceInfo, extract_instance_info};
use anyhow::{Error, Result, anyhow};
use futures::future::join_all;
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

    let mut instances = client
        .aggregated_list()
        .set_return_partial_success(true)
        .set_project(project_id)
        .by_item();

    let mut tasks = Vec::new();

    // Collect tasks instead of awaiting sequentially
    while let Some(result) = instances.next().await {
        let result = result.map_err(|e| format!("Error retrieving instance list: {e}"))?;
        let (zone, scoped_list) = result;

        if scoped_list.instances.is_empty() {
            continue;
        }

        // Clone what we need for async processing
        let zone_str = zone.clone();
        let instances_vec = scoped_list.instances;

        // Spawn async processing task
        tasks.push(tokio::spawn(async move {
            let mut infos = Vec::new();
            for instance in instances_vec {
                match extract_instance_info(&instance, &zone_str) {
                    Ok(info) => infos.push(info),
                    Err(err) => {
                        return Err(format!(
                            "Failed to process instance in zone {}: {err}",
                            zone_str
                        ));
                    }
                }
            }
            Ok::<Vec<InstanceInfo>, String>(infos)
        }));
    }

    // Run all tasks concurrently
    let results = join_all(tasks).await;

    // Flatten results
    let mut instances_array = Vec::new();
    for res in results {
        match res {
            Ok(Ok(mut infos)) => instances_array.append(&mut infos),
            Ok(Err(e)) => return Err(e),
            Err(e) => return Err(format!("Join error: {e}")),
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
