mod depth_scan;

pub fn call_verb(params: crate::AppParams) -> Result<(), String> {
    match params.verb.as_str() {
        "depth-scan" => depth_scan::run(params),
        _ => Err(format!("Verb {} not found!", params.verb)),
    }
}
