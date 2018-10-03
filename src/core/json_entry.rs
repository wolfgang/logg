use serde_json;

pub fn get_body_as_str(entry: &serde_json::Value) -> &str {
    entry["body"].as_str().unwrap()
}

pub fn get_id(entry: &serde_json::Value) -> i64 {
    entry["id"].as_i64().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_body_as_str_returns_body_as_string_ref() {
        let entry = json!({"body": "line1\nline2"});
        assert_eq!("line1\nline2", get_body_as_str(&entry));
    }

    #[test]
    fn get_id_returns_id_as_i64() {
        let entry = json!({"id": 1234});
        assert_eq!(1234, get_id(&entry));
    }
}