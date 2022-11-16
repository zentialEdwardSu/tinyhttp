use std::fmt::{Display,Formatter,Result as FmtResult};

#[derive(Clone, Copy,Debug)]
pub enum StatusCode{
    Ok = 200,
    BasRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_parse(&self) -> &str{
        match self {
            Self::Ok => "OK",
            Self::BasRequest => "BaaS Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}",*self as u16)
    }
}