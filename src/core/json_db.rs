use serde_json;
use chrono::prelude::*;


pub struct JsonDB<'a> {
    pub json: serde_json::Value,
    get_timestamp_fn: &'a Fn() -> i64

}

impl<'a> JsonDB<'a> {
    pub fn new(json: serde_json::Value) -> JsonDB<'a> {
        JsonDB { json, get_timestamp_fn: &get_timestamp }
    }

    fn _new(json: serde_json::Value, get_timestamp_fn: &'a Fn() -> i64) -> JsonDB<'a> {
        JsonDB { json, get_timestamp_fn }
    }

    pub fn add_entry(&mut self, cat: &str, body: &str) -> i64 {
        let mut the_id = 0;
        if self.json[cat].is_null() {
            self.json[cat] = json!({"entries": [new_entry(body, 0, (self.get_timestamp_fn)())]});
        }
        else {
            let entries_ref = self.json[cat]["entries"].as_array_mut().unwrap();
            the_id = entries_ref.len();
            entries_ref.push(new_entry(body, the_id, (self.get_timestamp_fn)()));
        }
        the_id as i64
    }

    pub fn replace_entry(&mut self, cat: &str, id: usize, new_body: &str) {
        let entries_ref = self.json[cat]["entries"].as_array_mut().unwrap();
        let created_ts = entries_ref[id]["created_ts"].as_i64().unwrap();
        entries_ref.remove(id);
        entries_ref.insert(id, updated_entry(new_body, id, created_ts, (self.get_timestamp_fn)()));
    }
}

fn new_entry(body: &str, id: usize, created_ts: i64) -> serde_json::Value {
    updated_entry(body, id, created_ts, created_ts)
} 

fn updated_entry(body: &str, id: usize, created_ts: i64, updated_ts: i64) -> serde_json::Value {
    json!({"body": body, "id": id, "created_ts": created_ts, "updated_ts": updated_ts})
} 

fn get_timestamp() -> i64 {
    let now: DateTime<Local> = Local::now();
    now.timestamp()
}


#[cfg(test)]
mod test {
    use super::*;

    const CONST_TIMESTAMP: i64 = 1234;

    fn get_timestamp_stub() -> i64 {
        CONST_TIMESTAMP
    }

    fn _db<'a>(json: serde_json::Value) -> JsonDB<'a> {
        JsonDB::_new(json, &get_timestamp_stub)
    }

    fn _body(text: &str, id: usize) -> serde_json::Value {
        json!({"body": text, "id": id, "created_ts": CONST_TIMESTAMP, "updated_ts": CONST_TIMESTAMP})
    }

    #[test]
    fn can_add_entry_to_empty_json() {
        let mut db = _db(json!({}));
        db.add_entry("some_category", "some_body");
        assert_eq!(
            json!({"some_category": {"entries": [_body("some_body", 0)]}}),
            db.json);
    }

    #[test]
    fn can_add_second_category() {
        let mut db =_db(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
        db.add_entry("category_2", "body_2");
        assert_eq!(
            json!({
                "category_1": {"entries": [_body("body_1", 0)]},
                "category_2": {"entries": [_body("body_2", 0)]}
            }),
            db.json);
    }
    #[test]
    fn can_add_entry_to_existing_category() {
        let mut db =_db(json!({"category_1": {"entries": [_body("body_1", 0)]}}));
        db.add_entry("category_1", "body_2");
        assert_eq!(
            json!({"category_1": {"entries": [_body("body_1", 0), _body("body_2", 1)]}}),
            db.json);
    }
    #[test]
    fn replace_entry_replaces_entry_with_given_id_with_given_bidy() {
        let mut db =_db(json!({
            "category_1": {"entries": [_body("body_1", 0), _body("body_2", 1)]}}));
        db.replace_entry("category_1", 0, "body_1_replaced");
        assert_eq!(
            json!({
                "category_1": {"entries": [_body("body_1_replaced", 0), _body("body_2", 1)]}}),
            db.json);
        db.replace_entry("category_1", 1, "body_2_replaced");

        assert_eq!(
            json!({
                "category_1": {"entries": [_body("body_1_replaced", 0), _body("body_2_replaced", 1)]}}),
            db.json);
    }

    #[test]
    fn add_entry_returns_id_of_just_added_entry() {
        let mut db = _db(json!({}));
        assert_eq!(0, db.add_entry("cat1", "body"));        
        assert_eq!(1, db.add_entry("cat1", "body"));        
        assert_eq!(0, db.add_entry("cat2", "body"));        
        assert_eq!(1, db.add_entry("cat2", "body"));        
    }


}