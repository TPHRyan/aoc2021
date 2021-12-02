mod depth_scan;
mod move_sub;

pub fn call_verb(params: crate::AppParams) -> Result<(), String> {
    match params.verb.as_str() {
        "depth-scan" => depth_scan::run(params),
        "move" => move_sub::run(params),
        _ => Err(format!("Verb {} not found!", params.verb)),
    }
}
