
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