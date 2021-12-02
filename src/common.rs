mod helpers;

pub use helpers::get_file_contents;
pub use helpers::int_lines;

pub struct AppParams {
    pub verb: String,
    pub data_file_path: String,
    pub data_file: std::fs::File,
}
