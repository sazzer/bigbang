use std::sync::Arc;

use super::{Configurer, Server};
use prometheus::Registry;

/// Component representing the HTTP Server
pub struct Component {
    pub server: Server,
}

/// Builder for building the HTTP Server component
#[derive(Default)]
pub struct Builder {
    config: Vec<Arc<dyn Configurer>>,
}

impl Builder {
    pub fn with_component(mut self, component: Arc<dyn Configurer>) -> Self {
        self.config.push(component);
        self
    }

    pub fn build(self, prometheus: Registry, port: u16) -> Component {
        Component {
            server: Server::new(port, self.config, prometheus),
        }
    }
}
