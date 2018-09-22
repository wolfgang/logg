extern crate serde_json;

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
    let data_str = "this is invalid json";

    let _: serde_json::Value = serde_json::from_str(data_str).unwrap();

}

#[test]
fn can_examine_error() {
    let data_str = "this is invalid json";

    serde_json::from_str(data_str).unwrap_or_else(|err| {
        assert!(err.is_syntax());
        assert_eq!(1, err.line());
    });

}

