use crate::Error;
use std::error::Error as ErrorTrait;
use std::str::FromStr;

#[derive(Debug)]
pub struct CaveNode {
    name: String,
    cave_type: CaveType,
}

impl CaveNode {
    pub fn start() -> CaveNode {
        CaveNode {
            name: String::from("start"),
            cave_type: CaveType::Start,
        }
    }

    pub fn end() -> CaveNode {
        CaveNode {
            name: String::from("end"),
            cave_type: CaveType::End,
        }
    }

    pub fn small(name: &str) -> CaveNode {
        assert_eq!(name, name.to_lowercase());
        CaveNode {
            name: String::from(name),
            cave_type: CaveType::Small,
        }
    }

    pub fn large(name: &str) -> CaveNode {
        assert_eq!(name, name.to_uppercase());
        CaveNode {
            name: String::from(name),
            cave_type: CaveType::Large,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn cave_type(&self) -> &CaveType {
        &self.cave_type
    }
}

impl FromStr for CaveNode {
    type Err = Box<dyn ErrorTrait>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(CaveNode::start()),
            "end" => Ok(CaveNode::end()),
            name => {
                if name == name.to_lowercase() {
                    Ok(CaveNode::small(name))
                } else if name == name.to_uppercase() {
                    Ok(CaveNode::large(name))
                } else {
                    Err(Box::new(Error::new(
                        "Cannot create cave from mixed case name!",
                    )))
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CaveType {
    End,
    Large,
    Small,
    Start,
}
