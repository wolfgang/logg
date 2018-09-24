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

}


pub fn by_body<'a>(_search_str: &str, json: &'a serde_json::Value) -> Vec<SearchResult<'a>> {

	let obj = json.as_object().unwrap();

    let mut  search_results = Vec::new();

	for cat_name in obj.keys() {
		let cat = &obj[cat_name];
		let entries = &cat["entries"];

        let mut sr = SearchResult::new(cat_name.to_string());

		for entry in entries.as_array().unwrap() {
            sr.add(entry)
		}

        search_results.push(sr);


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
                "cat1": {"entries": [{"body": "some body"}]}
            });
    	let results = by_body("", &json);
    	assert_eq!(1, results.len());
        let only_result = &results[0];
        assert_eq!("cat1", only_result.category);
        let matching_entries = &only_result.entries;
        assert_eq!(1, matching_entries.len());
        assert_eq!(&entry_with_body("some body"), matching_entries[0]);

    }

    fn entry_with_body(body: &str) -> serde_json::Value {
        json!({"body": body.to_string()})
    }


}
