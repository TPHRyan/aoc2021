mod common;
mod verbs;

use std::env;
use std::fmt::Write;
use std::fs::File;

use common::AppParams;

pub fn run(mut args: env::Args) -> Result<(), String> {
    let program_name = args.next().unwrap();
    let app_params = match parse_args(args) {
        Ok(params) => params,
        Err(message) => {
            let mut full_message = message.clone();
            return match writeln!(full_message, "Usage: {} <data-file-path>", program_name) {
                Ok(()) => Err(full_message),
                Err(err) => Err(format!("{}", err)),
            };
        }
    };
    return verbs::call_verb(app_params);
}

fn parse_args(mut args: env::Args) -> Result<AppParams, String> {
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
