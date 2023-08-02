use crate::query_db::search::Search;

trait ScraperApi {
    fn run_query(search: Search);
}
