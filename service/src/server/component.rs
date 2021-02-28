use super::Server;
use prometheus::Registry;

/// Component representing the HTTP Server
pub struct Component {
    pub server: Server,
}

impl Component {
    pub fn new(prometheus: Registry, port: u16) -> Self {
        Self {
            server: Server::new(port, prometheus),
        }
    }
}
