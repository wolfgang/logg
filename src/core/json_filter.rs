use serde_json;

pub struct SearchResult<'a> {
    pub category: String,
    pub entries: Vec<&'a serde_json::Value>
}

impl<'a> SearchResult<'a> {
    pub fn new(category: String) -> SearchResult<'a> {
        SearchResult { category, entries: Vec::new() }
    }

    pub fn add(&mut self, entry: &'a serde_json::Value) {
        self.entries.push(entry);
    }

    pub fn has_entries(&self) -> bool {
        self.entries.len() > 0
    }

}


pub fn by_body<'a>(search_str: &str, json: &'a serde_json::Value) -> Vec<SearchResult<'a>> {
	let obj = json.as_object().unwrap();
    let mut  search_results = Vec::new();

	for cat_name in obj.keys() {
		let entries = &obj[cat_name]["entries"];

        let mut sr = SearchResult::new(cat_name.clone());

		for entry in entries.as_array().unwrap() {
            let body = entry["body"].to_string();
            if body.contains(search_str) {                
                sr.add(entry)
            }
		}

        if sr.has_entries() {
            search_results.push(sr);
        }
	}
    search_results	
}



#[cfg(test)]
mod test {
	use super::*;
    #[test]
    fn search_result_construction() {
        let sr = SearchResult::new(String::from("some_category"));
        assert_eq!("some_category", sr.category);
        assert_eq!(0, sr.entries.len());
    }

    #[test]
    fn search_result_add_entries() {
        let entry1 = entry_with_body("some_body_1");
        let entry2 = entry_with_body("some_body_2");
        let mut sr = SearchResult::new(String::from("some_category"));
        sr.add(&entry1);
        sr.add(&entry2);
        assert_eq!(2, sr.entries.len());
        assert_eq!(&entry1, sr.entries[0]);
        assert_eq!(&entry2, sr.entries[1]);
    }


    #[test]
    fn return_full_input_if_search_is_empty_string() {        
    	let json = json!(
            {
                "cat1": {"entries": [{"body": "some body 1"}]},
                "cat2": {
                    "entries": [
                        {"body": "some body 2"},
                        {"body": "some body 3"}
                ]}
            });
    	let results = by_body("", &json);
    	assert_eq!(2, results.len());

        assert_result(&results, 0, "cat1", vec!(&entry_with_body("some body 1")));
        assert_result(
            &results, 1, "cat2", 
            vec!(&entry_with_body("some body 2"), &entry_with_body("some body 3")));
    }

    #[test]
    fn return_matching_entries_that_contain_given_word() {        
        let json = json!(
            {
                "cat1": {"entries": [{"body": "body with word1"}]},
                "cat2": {
                    "entries": [
                        {"body": "body with word2"},
                        {"body": "another body with word1"}
                ]}
            });
        let results = by_body("word1", &json);
        assert_eq!(2, results.len());

        assert_result(&results, 0, "cat1", 
            vec!(&entry_with_body("body with word1")));
        assert_result(&results, 1, "cat2",
            vec!(&entry_with_body("another body with word1")));
    }

    #[test]
    fn omit_categories_with_no_matching_entries() {        
        let json = json!(
            {
                "cat1": {"entries": [{"body": "body with word1"}]},
                "cat2": {"entries": [{"body": "body with word2"}]}
            });
        let results = by_body("word1", &json);
        assert_eq!(1, results.len());
        assert_result(&results, 0, "cat1", vec!(&entry_with_body("body with word1")));
    }



    fn assert_result(
        results: &Vec<SearchResult>,
        index: usize, 
        category: &str, 
        entries: Vec<&serde_json::Value>) {
        assert_eq!(category, results[index].category);
        let matching_entries = &results[index].entries;
        assert_eq!(entries, *matching_entries);

    }

    fn entry_with_body(body: &str) -> serde_json::Value {
        json!({"body": body.to_string()})
    }

}
