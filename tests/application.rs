use std::{error::Error, sync::Arc};

use rs_subito_alert::{
    application::{application_api::ApplicationApi, subito::Subito},
    query_db::search::Search,
};
use tokio::sync::Mutex;

use crate::test_doubles::{
    notifier::NotifierSpy,
    query::{QueryDbFake, QueryDbSpy},
    scraper::{ScraperFake, ScraperSpy},
};

mod test_doubles;

#[tokio::test]
async fn test_add_search() {
    let query_spy = Arc::new(Mutex::new(QueryDbSpy::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let mut subito = Subito::new(Arc::clone(&query_spy), Arc::clone(&scraper), Arc::clone(&notifier_spy));

    let _ = subito
        .add_search(String::from("Test"), String::from("test"))
        .await;

    assert_eq!(
        Arc::clone(&query_spy).lock().await.invocations,
        vec![Arc::new(Search::new(
            String::from("Test"),
            String::from("test")
        ))]
    )
}

#[tokio::test]
async fn test_delete_search() {
    let query_spy = Arc::new(Mutex::new(QueryDbSpy::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let mut subito = Subito::new(Arc::clone(&query_spy), Arc::clone(&scraper), Arc::clone(&notifier_spy));

    let _ = subito.delete_search(String::from("Test")).await;

    assert_eq!(Arc::clone(&query_spy).lock().await.deletions, vec![String::from("Test")])
}

#[tokio::test]
async fn test_list_search() {
    let query_spy = Arc::new(Mutex::new(QueryDbSpy::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(Arc::clone(&query_spy), Arc::clone(&scraper), Arc::clone(&notifier_spy));

    let _ = subito.list();

    assert_eq!(Arc::clone(&query_spy).lock().await.lists, vec![()])
}

#[tokio::test]
async fn test_scrape() -> Result<(), Box<dyn Error>> {
    let query_spy = Arc::new(Mutex::new(QueryDbSpy::new()));
    let scraper_spy = Arc::new(ScraperSpy::new());
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(Arc::clone(&query_spy), Arc::clone(&scraper_spy), Arc::clone(&notifier_spy));

    let _ = subito.scrape().await;

    assert_eq!(*scraper_spy.invocations.lock().await, 3);
    Ok(())
}

#[tokio::test]
async fn test_scrape_results() -> Result<(), Box<dyn Error>> {
    let query_spy = Arc::new(Mutex::new(QueryDbSpy::new()));
    let scraper_spy = Arc::new(ScraperSpy::new());
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(Arc::clone(&query_spy), Arc::clone(&scraper_spy), Arc::clone(&notifier_spy));

    let results = subito.scrape().await?;

    assert_eq!(*scraper_spy.invocations.lock().await, (results.len() as i32));
    Ok(())
}

#[tokio::test]
async fn test_notification_on_new_items() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbFake::new()));
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(Arc::clone(&query_fake), Arc::clone(&scraper_spy), Arc::clone(&notifier_spy));

    let results = subito.scrape().await?;

    assert_eq!(*notifier_spy.invocations.lock().await, (results.len() as i32) - 2);
    Ok(())
}
