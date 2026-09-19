use {crate::traits::QueryApp, std::sync::Arc};

#[derive(Clone)]
pub struct Context {
    pub bolt_app: Arc<dyn QueryApp + Send + Sync>,
}

impl Context {
    pub fn new(bolt_app: Arc<dyn QueryApp + Send + Sync>) -> Self {
        Self { bolt_app }
    }
}
