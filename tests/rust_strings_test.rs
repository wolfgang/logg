
#[test]
fn split_by_newlines() {
	let parts: Vec<&str> = "line1\nline2\nline3".split('\n').collect();
    assert_eq!(vec!("line1", "line2", "line3"), parts);
}

#[test] 
fn remove_enclosing_quotes() {
	let s = "\"in quotes\"";
	let trimmed = s.trim_matches('"');
	assert_eq!("in quotes", trimmed);
}

#[test]
fn join_strings_from_array() {
	let args: Vec<String> = vec!(String::from("word1"), String::from("word2"));
	let args_ref: &[String] = &args;
	assert_eq!("word1 word2", args_ref.join(" "));



}