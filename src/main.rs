use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap();
    match parse_args(args) {
        Ok(app_params) => {
            println!("Success!");
            println!("Verb: {}", app_params.verb);
            println!("Data file: {}", app_params.data_file_path);
        },
        Err(message) => {
            println!("{}", message);
            println!("Usage: {} <data-file-path>", program_name)
        }
    }
}

fn parse_args(mut args: env::Args) -> Result<AppParams, String> {
    let verb = match args.next() {
        Some(arg) => arg,
        None => return Err("Verb not provided!".to_string())
    };
    let data_file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Data file path not provided!".to_string())
    };
    let data_file = match fs::File::open(&data_file_path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string())
    };
    Ok(AppParams {
        verb,
        data_file_path,
        data_file
    })
}

struct AppParams {
    verb: String,
    data_file_path: String,
    data_file: fs::File,
}