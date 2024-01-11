use std::collections::HashMap;

pub mod permission;
pub mod datetime;

pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split("&")
        .filter_map(|param| {
            let mut parts = param.splitn(2, "=");
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}