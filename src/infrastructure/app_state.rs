use minijinja::Environment;

#[derive(Clone, Debug)]
pub struct AppState {
    pub template_env: Environment<'static>,
}
