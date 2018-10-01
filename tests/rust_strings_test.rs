
#[test]
fn split_by_newlines() {
	let parts: Vec<&str> = "line1\nline2\nline3".split('\n').collect();
    assert_eq!(vec!("line1", "line2", "line3"), parts);
}

// use std::fs;
// #[test]
// fn split_by_newlines_from_file() {
// 	let contents = fs::read_to_string("/tmp/logg_tmp.txt").expect("Failed to read file contents");
// 	let parts: Vec<&str> = contents.split('\n').collect();
// 	assert_eq!(vec!("This is entry line 1",  "And this is line 2", "Line 3 is here", ""), parts );
// }
