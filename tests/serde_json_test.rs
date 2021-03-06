#[macro_use]
extern crate serde_json;

const INVALID_JSON: &'static str = "this is invalid json";

const VALID_JSON: &'static str = 
    r#"{
        "some_str": "John",
        "some_int": 1234,
        "some_list": [1, 2]
    }
    "#;

const SIMPLE_VALID_JSON: &'static str = r#"{"key":1234}"#;


#[test]
fn from_str_to_untyped() {
    let v: serde_json::Value = serde_json::from_str(VALID_JSON).unwrap();
    assert_eq!("John", v["some_str"]);
    assert_eq!(1234, v["some_int"]);
    assert_eq!(1, v["some_list"][0]);
    assert_eq!(2, v["some_list"][1]);

    let l = &v["some_list"];
    assert_eq!(1, l[0]);
    assert_eq!(2, l[1]);
}

#[test]
#[should_panic]
fn panics_if_json_is_invalid() {
    let _: serde_json::Value = serde_json::from_str(INVALID_JSON).unwrap();
}

#[test]
fn can_examine_error() {
    serde_json::from_str(INVALID_JSON).unwrap_or_else(|err| {
        assert!(err.is_syntax());
        assert_eq!(1, err.line());
    });
}

#[test]
fn can_propagate_error() {
    let result = try_from_str(VALID_JSON);

    let value = match result {
        Ok(value) => value,
        Err(error) => {
            panic!("Could not parse string: {:?}", error)
        },
    };

    assert_eq!("John", value["some_str"]);
}

#[test]
fn convert_json_to_string() {
    let json: serde_json::Value = serde_json::from_str(SIMPLE_VALID_JSON).unwrap();
    let json_string = json.to_string();
    assert_eq!(SIMPLE_VALID_JSON, json_string);
}

#[test]
fn construct_json_with_macro() {
    let json = json!({
        "key1": 1234,
        "key2": "abcd" 
    });
    assert_eq!(1234, json["key1"]);
    assert_eq!("abcd", json["key2"]);
}

#[test]
fn test_if_contains_key() {
    let json = json!({
        "key1": 1234,
        "key2": "abcd" 
    });
    assert!(!json["key1"].is_null());
    assert!(json["key3"].is_null());
}

#[test]
fn add_to_array() {
    let mut json = json!({
        "arr": ["aaaa"]
    });

    let arr_ref = &mut json["arr"].as_array_mut().unwrap();
    assert_eq!("aaaa", arr_ref[0]);
    arr_ref.push(serde_json::Value::String("bbbb".to_string()));
    assert_eq!("bbbb", arr_ref[1]);

}

fn try_from_str(data_str: &str) -> Result<serde_json::Value, serde_json::Error>{
    let v: serde_json::Value = serde_json::from_str(data_str)?;
    Ok(v)

}

