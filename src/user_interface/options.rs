use std::fmt::{Display, Formatter, Result};

pub enum Options {
    Start,
    Quit,
    ApiKey,
}

impl Display for Options {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Start => write!(f, "Start application"),
            Self::Quit => write!(f, "Quit application"),
            Self::ApiKey => write!(f, "Update telegram api_key"),
        }
    }
}
