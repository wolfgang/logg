extern crate serde_json;

const INVALID_JSON: &'static str = "this is invalid json";

const VALID_JSON: &'static str = 
    r#"{
        "some_str": "John",
        "some_int": 1234,
        "some_list": [1, 2]
    }
    "#;


#[test]
fn from_str_to_untyped() {
    let data_str = r#"{
    	"some_str": "John",
    	"some_int": 1234,
    	"some_list": [1, 2]
    }
    "#;

    let v: serde_json::Value = serde_json::from_str(data_str).unwrap();
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

fn try_from_str(data_str: &str) -> Result<serde_json::Value, serde_json::Error>{
    let v: serde_json::Value = serde_json::from_str(data_str)?;
    Ok(v)

}

