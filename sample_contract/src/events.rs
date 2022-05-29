use near_sdk::{log};
use serde_json::{json};


fn log_basic_event_format(standard: &str, version: &str, event_type: &str, data_vec: Vec<&str>) {
    log!("EVENT_JSON:{}", &json!({
        "standard": standard, 
        "version": version, 
        "event": event_type,
        "data": data_vec
    }).to_string())
}