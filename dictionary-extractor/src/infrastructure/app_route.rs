#[derive(Debug, Clone)]
pub enum AppRoute {
    Home,
    Search,
}

impl AppRoute {
    pub fn pattern(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Search => "/search",
        }
    }
}

pub fn build_url(route: AppRoute) -> String {
    match route {
        AppRoute::Home => route.pattern().to_string(),
        AppRoute::Search => route.pattern().to_string(),
    }
}
