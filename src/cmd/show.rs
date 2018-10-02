use serde_json;


pub(super) fn execute(args: &[String]) {
	let json: serde_json::Value = ::core::io::get_file_contents_as_json();
	let results = ::core::json_filter::by_body("", &json);
	for result in results {
		if args.len()==2 {
			println!("> {} ({})", result.category, result.entries.len());
		}
		else {
			let cat = &args[2];
			if cat == &result.category {
				if args.len()==4 {
					let index = args[3].parse::<usize>().unwrap();
					let body_as_str =  ::core::json::get_body_as_str(result.entries[index]);
					println!("> {}[{}]\n----------\n{}", cat, index, body_as_str);					
				}
				else {
					println!("> {}", cat);
					let mut index = 0;
					for entry in result.entries {
						let body_as_str =  ::core::json::get_body_as_str(&entry);
						let lines: Vec<&str> = body_as_str.lines().collect();
						let more = if lines.len() > 1 { "[...]" } else { "" };
						println!("[{}] {} {}", index, lines[0], more);
						index = index + 1;
					}
				}
			}
		}
	}
}
