mod common;
mod verbs;

use std::env;
use std::fs::File;

use common::AppParams;

pub fn run(app_params: AppParams) -> Result<(), String> {
    return verbs::call_verb(app_params);
}

pub fn parse_args(mut args: env::Args) -> Result<AppParams, String> {
    let verb = match args.next() {
        Some(arg) => arg,
        None => return Err(String::from("Verb not provided!")),
    };
    let data_file_path = match args.next() {
        Some(arg) => arg,
        None => return Err(String::from("Data file path not provided!")),
    };
    let data_file = match File::open(&data_file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("{}", e)),
    };
    Ok(AppParams {
        verb,
        data_file_path,
        data_file,
    })
}
