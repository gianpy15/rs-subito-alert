use std::error::Error;

use rs_subito_alert::{
    query_db::search::Search,
    scraper::{
        downloader::download_agent::DownloadAgent, scraper_agent::ScraperAgent,
        scraper_api::ScraperApi,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let download: DownloadAgent = Default::default();
    let mut scraper = ScraperAgent::new(&download);

    let results = scraper.run_query(
        Search {
            name: "Test".to_string().into(),
            query: "Zelda Tears of the kingdom".to_string().into(),
        }
        .into(),
    )?;

    for result in results {
        println!("{}", result)
    }

    Ok(())
}
