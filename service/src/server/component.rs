use super::Server;
use prometheus::Registry;

/// Component representing the HTTP Server
pub struct Component {
    pub server: Server,
}

/// Builder for building the HTTP Server component
pub struct Builder {
    prometheus: Registry,
}

impl Component {
    pub fn builder(prometheus: Registry) -> Builder {
        Builder { prometheus }
    }
}

impl Builder {
    pub fn build(self) -> Component {
        Component {
            server: Server::new(8000, self.prometheus),
        }
    }
}
