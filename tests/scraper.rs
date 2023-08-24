use std::{error::Error, sync::Arc};

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{scraper_agent::ScraperAgent, scraper_api::ScraperApi},
};

use crate::test_doubles::scraper::DownloadFake;

mod test_doubles;

#[tokio::test]
async fn test_scraping() -> Result<(), Box<dyn Error>> {
    let fake_download = Arc::new(DownloadFake::new());
    let agent = ScraperAgent::new(Arc::clone(&fake_download));

    let results = agent
        .run_query(
            Search {
                name: "Test".to_string().into(),
                query: "test".to_string().into(),
            }
            .into(),
        )
        .await?;

    assert_eq!(results.len(), 30);

    Ok(())
}
