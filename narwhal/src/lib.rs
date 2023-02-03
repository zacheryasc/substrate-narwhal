use std::str::FromStr;

pub mod error;
pub mod primary;
pub mod traits;
pub mod types;
pub mod worker;

#[derive(Debug, Clone)]
pub enum Role {
    Worker,
    Primary,
}

#[derive(Debug)]
pub struct RoleParseError;

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Role::*;

        match s.to_ascii_lowercase().as_str() {
            "worker" => Ok(Worker),
            "primary" => Ok(Primary),
            _ => Err(RoleParseError),
        }
    }
}
