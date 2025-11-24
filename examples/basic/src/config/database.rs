use luminos_config::{env, Node, node};

pub fn config() -> Vec<(&'static str, Node)> {
    vec![
        ("default", env("DB_CONNECTION", "sqlite".to_string())),

        ("connections", node![
            ("sqlite", node![
                ("driver", "sqlite"),
                ("database", env("DB_DATABASE", "database.sqlite".to_string())),
                ("prefix", ""),
                ("foreign_key_constraints", env("DB_FOREIGN_KEYS", true)),
            ]),
            ("mysql", node![
                ("driver", "mysql"),
                ("database", env("DB_DATABASE", "localhost".to_string())),
                ("port", env("DB_PORT", "1234".to_string())),
                ("prefix", ""),
                ("foreign_key_constraints", env("DB_FOREIGN_KEYS", true)),
            ]),
        ]),

        ("migrations", node![
            ("table", "migrations"),
            ("update_on_publish", true),
        ]),
    ]
}
