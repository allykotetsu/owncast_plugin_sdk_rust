#[derive(Eq, Hash, PartialEq)]
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