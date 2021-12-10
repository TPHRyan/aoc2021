mod helpers;

use core::result::Result as CoreResult;
use std::error::Error as ErrorTrait;
use std::fmt::{Debug, Display, Formatter};

pub use helpers::bit_lines;
pub use helpers::first_line;
pub use helpers::int_lines;
pub use helpers::split_lines_on;

pub struct AppParams {
    pub program_name: String,
    pub use_example_data: bool,
    pub challenge_day: Option<u32>,
    pub challenge_part: Option<u32>,
}

pub type Result<T> = CoreResult<T, Box<dyn ErrorTrait>>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new<T: ?Sized>(message: &T) -> Error
    where
        T: ToString,
    {
        Error {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ErrorTrait for Error {}
