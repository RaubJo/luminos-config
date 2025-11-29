use luminos_config::{Node, env};

pub fn config() -> Vec<(&'static str, Node)> {
    vec![
        ("name", env("APP_NAME", "Example".to_string())),
        ("env", env("APP_ENV", "production".to_string())),
        ("debug", env("APP_DEBUG", false)),
    ]
}
