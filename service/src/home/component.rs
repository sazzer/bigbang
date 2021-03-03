use super::{HomeLinksUseCase, LinkContributor};
use crate::server::Configurer;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// Component representing the home document
pub struct Component {
    service: Arc<HomeLinksUseCase>,
}

/// Builder to build the home document component
#[derive(Default)]
pub struct Builder {
    contributors: Vec<Arc<dyn LinkContributor>>,
}

impl Builder {
    /// Add a new contributor of links to the home document.
    #[allow(dead_code)]
    pub fn with_contributor(mut self, contributor: Arc<dyn LinkContributor>) -> Self {
        self.contributors.push(contributor);

        self
    }

    /// Build the actual home document component.
    pub fn build(self) -> Arc<Component> {
        let mut contributors = self.contributors;
        contributors.push(Arc::new(vec![("self".to_owned(), "/".into())]));

        let service = Arc::new(HomeLinksUseCase { contributors });

        Arc::new(Component { service })
    }
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
        super::http::configure_server(config);
    }
}
