use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;


pub fn get_contents() -> String {
    let file_name = "/tmp/logg_tmp.txt";
        if Path::new(file_name).exists() {
            fs::remove_file(file_name).expect("Failed to remove temp file");
        }
        let editor = env::var("EDITOR").expect("EDITOR variable is not set");
        let status = Command::new(editor)
            .arg(file_name)
            .status()
            .expect("Failed to execute editor");

        println!("vi exited with: {}", status);
        let contents = fs::read_to_string(file_name).expect("Failed to read file contents");
        println!("edited file contents: {}", contents);
        contents
}

        