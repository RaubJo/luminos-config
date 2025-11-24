use std::str::FromStr;
use crate::Node;

pub fn env<T>(key: &str, default: T) -> Node
where
    T: FromStr + Into<Node> + Clone,
{
    match std::env::var(key) {
        Ok(val_str) => match val_str.parse::<T>() {
            Ok(parsed) => parsed.into(),
            Err(_) => default.clone().into(),
        },
        Err(_) => default.into(),
    }
}
