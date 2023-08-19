use std::error::Error;

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{scraper_agent::ScraperAgent, scraper_api::ScraperApi},
};

use crate::test_doubles::scraper::DownloadFake;

mod test_doubles;

#[test]
fn test_scraping() -> Result<(), Box<dyn Error>> {
    let fake_download = DownloadFake::new();
    let mut agent = ScraperAgent::new(&fake_download);

    let results = agent.run_query(
        Search {
            name: "Test".to_string().into(),
            query: "test".to_string().into(),
        }
        .into(),
    )?;

    assert_eq!(results.len(), 30);

    Ok(())
}
