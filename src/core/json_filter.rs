use serde_json;
use core::{json_db::JsonDB, json_entry};

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

    pub fn is_unqiue(&self) -> bool {
        return self.entries.len() == 1;
    }

    pub fn is_valid_id(&self, id: usize) -> bool {
        return id < self.entries.len();
    }

    pub fn get_body_by_id(&self, id: usize) -> &str {
        let entry = &self.entries[id];
        json_entry::get_body_as_str(entry)
    }
}

pub trait Filter {
    fn filter_by_body(&self, search_str: &str) -> Vec<SearchResult>;
    fn filter_by_category(&self, category: &str) -> SearchResult;
}


impl<'a> Filter for JsonDB<'a> {
    fn filter_by_body(&self, search_str: &str) -> Vec<SearchResult> {
        by_body(search_str, &self.json)
    }
    fn filter_by_category(&self, category: &str) -> SearchResult {
        by_category(category, &self.json)
    }
}






fn by_body<'a>(search_str: &str, json: &'a serde_json::Value) -> Vec<SearchResult<'a>> {
	let obj = json.as_object().unwrap();
    let mut  search_results = Vec::new();
    let search_str_lowercase = search_str.to_lowercase();

	for cat_name in obj.keys() {
		let entries = &obj[cat_name]["entries"];
        let mut sr = SearchResult::new(cat_name.clone());

		for entry in entries.as_array().unwrap() {
            let body = entry["body"].as_str().unwrap().to_lowercase();
            if body.contains(&search_str_lowercase) {                
                sr.add(entry)
            }
		}

        if sr.has_entries() {
            search_results.push(sr);
        }
	}
    search_results	
}

fn by_category<'a>(category: &str, json: &'a serde_json::Value) -> SearchResult<'a> {
    let mut result = SearchResult::new(String::from(category));
    let obj = json.as_object().unwrap();

    if !obj.contains_key(category) {
        return result
    }

    let entries = &obj[category]["entries"];
    for entry in entries.as_array().unwrap() {
        result.add(entry)
    }

    result
}



#[cfg(test)]
mod test {
	use super::*;

    #[test]
    fn by_category_returns_category() {
        let json = json!(
            {
                "cat1": {"entries": [{"body": "body with word1"}]},
                "cat2": {"entries": [{"body": "body with word2"}]}
            });
        let result1 = by_category("cat1", &json);
        let result2= by_category("cat2", &json);
        assert_result(&result1, "cat1", vec!(&entry_with_body("body with word1")));
        assert_result(&result2, "cat2", vec!(&entry_with_body("body with word2")));
    }

    #[test]
    fn by_category_returns_empty_result_if_category_does_not_exist() {
        let json = json!({});
        let result = by_category("cat1", &json);
        assert!(!result.has_entries());
    }

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
    fn search_result_is_unique() {
        let entry1 = entry_with_body("some_body_1");
        let entry2 = entry_with_body("some_body_2");
        let mut sr = SearchResult::new(String::from("some_category"));
        sr.add(&entry1);
        assert!(sr.is_unqiue());
        sr.add(&entry2);
        assert!(!sr.is_unqiue());

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

        assert_result_in(&results, 0, "cat1", vec!(&entry_with_body("some body 1")));
        assert_result_in(
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

        assert_result_in(&results, 0, "cat1", 
            vec!(&entry_with_body("body with word1")));
        assert_result_in(&results, 1, "cat2",
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
        assert_result_in(&results, 0, "cat1", vec!(&entry_with_body("body with word1")));
    }

    #[test]
    fn search_is_case_insensitive() {
        let json = json!({"cat1": {"entries": [{"body": "some body"}]}});
        let results = by_body("SoMe", &json);
        assert_eq!(1, results.len());
    }

    fn assert_result_in(
        results: &Vec<SearchResult>, 
        index: usize, 
        category: &str, 
        entries: Vec<&serde_json::Value>) 
    {
        assert_result(&results[index], category, entries);
    }

    fn assert_result(result: &SearchResult, category: &str, entries: Vec<&serde_json::Value>) {
        assert_eq!(category, result.category);
        let matching_entries = &result.entries;
        assert_eq!(&entries, matching_entries);
    }


    fn entry_with_body(body: &str) -> serde_json::Value {
        json!({"body": body.to_string()})
    }

}
