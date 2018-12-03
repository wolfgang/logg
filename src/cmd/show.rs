use core::{json_filter, display};
use core::error::*;
use cmd::utils;

pub(super) fn execute(args: &[String]) -> EmptyBoxedResult {
    let db = utils::create_db_from_log();

    if args.len()==2 {
        display::show_toc(&db);
        return Ok(())
    }

    let result = utils::get_requested_category(args, &db)?;

    if args.len()==4 {
        let id = utils::parse_requested_id(args)?;
        show_search_result_entry_with_id(&result, id)?;
        return Ok(());
    }
    
    if result.is_unqiue() {
        display::show_entry_for_search_result(&result, 0);
    }
    else {
        display::show_toc_for_search_result(&result);
    }

    Ok(())
}

fn show_search_result_entry_with_id(result: &json_filter::SearchResult, id: usize) -> EmptyBoxedResult {
    utils::check_is_valid_id(id, &result)?;
    display::show_entry_for_search_result(&result, id);
    Ok(())
}