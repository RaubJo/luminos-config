use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Node {
    Null,
    Bool(bool),
    Number(i64),
    Text(String),
    Object(HashMap<String, Node>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Object(HashMap::new())
    }
}

impl From<&[(&str, Node)]> for Node {
    fn from(slice: &[(&str, Node)]) -> Node {
        Node::Object(
            slice
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }
}

impl<T> From<Vec<(&str, T)>> for Node
where
    T: Into<Node>,
{
    fn from(v: Vec<(&str, T)>) -> Self {
        Node::Object(
            v.into_iter()
                .map(|(k, val)| (k.to_string(), val.into()))
                .collect(),
        )
    }
}

impl From<bool> for Node {
    fn from(b: bool) -> Self {
        Node::Bool(b)
    }
}

impl From<i64> for Node {
    fn from(n: i64) -> Self {
        Node::Number(n)
    }
}

impl From<String> for Node {
    fn from(s: String) -> Self {
        Node::Text(s)
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Node::Text(s.to_string())
    }
}

#[macro_export]
macro_rules! node {
    // Match: [ (key, value), ... ]
    [ $( ($key:expr, $value:expr) ),* $(,)? ] => {{
        let v: Vec<(&str, $crate::Node)> = vec![
            $(($key, {
                let val: $crate::Node = $value.into();
                val
            })),*
        ];
        let result: $crate::Node = v.into();
        result
    }};
}
