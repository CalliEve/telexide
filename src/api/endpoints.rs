use http::Method;

pub enum APIEndpoint {
    GetUpdates,
    GetMe,
    SendMessage,
    SetMyCommands,
    GetMyCommands,
}

impl APIEndpoint {
    pub fn as_str(&self) -> &str {
        match *self {
            Self::GetUpdates => "getUpdates",
            Self::GetMe => "getMe",
            Self::SendMessage => "sendMessage",
            Self::SetMyCommands => "setMyCommands",
            Self::GetMyCommands => "getMyCommands",
        }
    }

    pub fn get_method(&self) -> Method {
        match *self {
            Self::GetUpdates => Method::GET,
            Self::GetMe => Method::GET,
            Self::SendMessage => Method::POST,
            Self::GetMyCommands => Method::GET,
            Self::SetMyCommands => Method::POST,
        }
    }
}

impl std::fmt::Display for APIEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for APIEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("telegram::APIEndpoint")
            .field(&self.as_str().to_owned())
            .finish()
    }
}
