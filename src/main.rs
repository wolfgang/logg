
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::env;

extern crate serde_json;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let file = OpenOptions::new().append(true).open("logg.txt")?;
    let mut file = BufWriter::new(file);

    let s = format!("{}\n", args[1]);

    file.write_all(s.as_bytes())?;

    Ok(())

}

#[cfg(test)]
mod tests {
	use serde_json;
	
    #[test]
    fn serde_json_smoketest() {
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
}