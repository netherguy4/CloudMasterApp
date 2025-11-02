use anyhow::Result;
use google_cloud_compute_v1::model::Instance;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(serde::Serialize)]
struct JsonResponse<T: Serialize> {
    status: String,
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

pub fn convert_to_json_response<T: Serialize>(
    status: &str,
    code: u16,
    data: Option<T>,
    message: Option<String>,
) -> String {
    let response = JsonResponse {
        status: status.to_string(),
        code,
        message,
        data,
    };

    serde_json::to_string_pretty(&response).unwrap_or_else(|_| {
        json!({
            "status": "error",
            "code": 500,
            "message": "Failed to serialize response",
            "data": Value::Null
        })
        .to_string()
    })
}

pub fn success_response<T: Serialize>(data: T) -> String {
    convert_to_json_response("success", 200, Some(data), None)
}

pub fn error_response(code: u16, message: &str) -> String {
    convert_to_json_response::<Value>("error", code, None, Some(message.to_string()))
}

#[derive(Serialize, Debug)]
pub struct InstanceInfo {
    name: String,
    id: String,
    status: String,
    machine_type: String,
    internal_ips: Vec<String>,
    external_ips: Vec<String>,
    zone: String,
}

pub fn extract_instance_info(instance: &Instance, zone: &str) -> Result<InstanceInfo> {
    let name = instance.name.clone().unwrap_or_else(|| "<unknown>".into());
    let status = instance
        .status
        .clone()
        .unwrap_or_else(|| "<no status>".into())
        .to_string();
    let machine_type = instance
        .machine_type
        .as_ref()
        .and_then(|s| s.split('/').last())
        .unwrap_or("<unknown>")
        .to_string();
    let id = instance
        .id
        .map(|v| v.to_string())
        .unwrap_or_else(|| "<no id>".into());

    let mut internal_ips = Vec::new();
    let mut external_ips = Vec::new();

    for nic in &instance.network_interfaces {
        if let Some(ip) = &nic.network_ip {
            internal_ips.push(ip.clone());
        }
        for cfg in &nic.access_configs {
            if let Some(ip) = &cfg.nat_ip {
                external_ips.push(ip.clone());
            }
        }
    }

    Ok(InstanceInfo {
        name,
        id,
        status,
        machine_type,
        internal_ips,
        external_ips,
        zone: zone.to_string(),
    })
}
