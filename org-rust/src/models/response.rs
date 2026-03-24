use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tip<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> Tip<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }
}

impl Tip<serde_json::Value> {
    pub fn error(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

