mod config;
use luminos_config::{Config, Node};

fn main() {
    let cfg: Config = config::load_config();

    if let Some(Node::Text(name)) = cfg.get("app.name") {
        println!("App name: {}", name);
    }


    if let Some(Node::Text(env)) = cfg.get("app.env") {
        println!("App env: {}", env);
    }

    if let Some(Node::Bool(debug)) = cfg.get("app.debug") {
        println!("Debug mode: {}", debug);
    }

    if let Some(Node::Text(default_connection)) = cfg.get("database.default") {
        println!("Database connection: {}", default_connection);

        // let driver_path = format!("database.connections.{}.driver", default_connection);
        if let Some(Node::Text(driver)) = cfg.get(&format!("database.connections.{}.driver", default_connection)) {
            println!("Database driver: {}", driver);

            if driver == "mysql" {
                if let Some(Node::Text(port)) = cfg.get(&format!("database.connections.{}.port", default_connection)) {
                    println!("Database Port: {}", port);
                } 
            }
        }
    }
}
