use std::sync::Arc;

use actix_web::web::ServiceConfig;

use crate::server::Configurer;

/// Component representing the home document
pub struct Component {}

#[derive(Default)]
pub struct Builder {}

impl Builder {
    #[allow(clippy::unused_self)]
    pub fn build(self) -> Arc<Component> {
        Arc::new(Component {})
    }
}

impl Configurer for Component {
    fn configure_server(&self, _config: &mut ServiceConfig) {}
}
