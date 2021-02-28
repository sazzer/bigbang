use super::Server;
use prometheus::Registry;

/// Component representing the HTTP Server
pub struct Component {
    pub server: Server,
}

/// Builder for building the HTTP Server component
pub struct Builder {
    prometheus: Registry,
    port: u16,
}

impl Component {
    pub fn builder(prometheus: Registry, port: u16) -> Builder {
        Builder { prometheus, port }
    }
}

impl Builder {
    pub fn build(self) -> Component {
        Component {
            server: Server::new(self.port, self.prometheus),
        }
    }
}
