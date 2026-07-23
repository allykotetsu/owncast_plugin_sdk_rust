use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATH,
    POST,
    PUT,
    TRACE
}

impl TryFrom<&String> for Method {
    type Error = ();

    fn try_from(string: &String) -> Result<Self, Self::Error> {
        match string.as_str() {
            "CONNECT" => Ok(Self::CONNECT),
            "DELETE" => Ok(Self::DELETE),
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATH" => Ok(Self::PATH),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(())
        }
    }
}

impl Into<String> for &Method {
    fn into(self) -> String {
        match self {
            Method::CONNECT => "CONNECT".to_string(),
            Method::DELETE => "DELETE".to_string(),
            Method::GET => "GET".to_string(),
            Method::HEAD => "HEAD".to_string(),
            Method::OPTIONS => "OPTIONS".to_string(),
            Method::PATH => "PATH".to_string(),
            Method::POST => "POST".to_string(),
            Method::PUT => "PUT".to_string(),
            Method::TRACE => "TRACE".to_string()
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(self))
    }
}