use std::{error::Error, sync::Arc};

use rs_subito_alert::{
    application::{application_api::ApplicationApi, subito::Subito},
    query_db::search::Search,
};
use tokio::sync::Mutex;

use crate::test_doubles::{
    notifier::NotifierSpy,
    query::QueryDbDouble,
    scraper::{ScraperFake, ScraperSpy},
};

mod test_doubles;

#[tokio::test]
async fn test_add_search() {
    let query_spy = Arc::new(Mutex::new(QueryDbDouble::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let mut subito = Subito::new(
        Arc::clone(&query_spy),
        Arc::clone(&scraper),
        Arc::clone(&notifier_spy),
    );

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
    let query_spy = Arc::new(Mutex::new(QueryDbDouble::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let mut subito = Subito::new(
        Arc::clone(&query_spy),
        Arc::clone(&scraper),
        Arc::clone(&notifier_spy),
    );

    let _ = subito.delete_search(String::from("Test")).await;

    assert_eq!(
        Arc::clone(&query_spy).lock().await.deletions,
        vec![String::from("Test")]
    )
}

#[tokio::test]
async fn test_list_search() {
    let query_spy = Arc::new(Mutex::new(QueryDbDouble::new()));
    let scraper = Arc::new(ScraperFake {});
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_spy),
        Arc::clone(&scraper),
        Arc::clone(&notifier_spy),
    );

    let _ = subito.list().await;

    assert_eq!(*Arc::clone(&query_spy).lock().await.lists.lock().await, 1)
}

#[tokio::test]
async fn test_scrape() -> Result<(), Box<dyn Error>> {
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake.lock().await.set_items(vec![
        Arc::new(String::from("test")),
        Arc::new(String::from("test2")),
    ]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test".to_string(), "test".to_string())),
        Arc::new(Search::new("Test2".to_string(), "test2".to_string())),
        Arc::new(Search::new("Test3".to_string(), "test3".to_string())),
    ]);

    let scraper_spy = Arc::new(ScraperSpy::new());
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let _ = subito.scrape().await;

    assert_eq!(*scraper_spy.invocations.lock().await, 3);
    Ok(())
}

#[tokio::test]
async fn test_scrape_results() -> Result<(), Box<dyn Error>> {
    let query_spy = Arc::new(Mutex::new(QueryDbDouble::new()));
    let scraper_spy = Arc::new(ScraperSpy::new());
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_spy),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let results = subito.scrape().await?;

    assert_eq!(
        *scraper_spy.invocations.lock().await,
        (results.len() as i32)
    );
    Ok(())
}

#[tokio::test]
async fn test_notification_on_new_items() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake.lock().await.set_items(vec![
        Arc::new(String::from("test")),
        Arc::new(String::from("test2")),
    ]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test".to_string(), "test".to_string())),
        Arc::new(Search::new("Test2".to_string(), "test2".to_string())),
        Arc::new(Search::new("Test3".to_string(), "test3".to_string())),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let results = subito.scrape().await?;

    assert_eq!(
        *notifier_spy.invocations.lock().await,
        (results.len() as i32) - 2
    );
    Ok(())
}

#[tokio::test]
async fn test_new_items_are_added_to_db() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake.lock().await.set_items(vec![
        Arc::new(String::from("test")),
        Arc::new(String::from("test2")),
    ]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test".to_string(), "test".to_string())),
        Arc::new(Search::new("Test2".to_string(), "test2".to_string())),
        Arc::new(Search::new("Test3".to_string(), "test3".to_string())),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let _results = subito.scrape().await?;

    assert_eq!(*query_fake.lock().await.adds, vec!["test3"]);
    Ok(())
}

#[tokio::test]
async fn test_add_user() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake.lock().await.set_items(vec![
        Arc::new(String::from("test")),
        Arc::new(String::from("test2")),
    ]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test".to_string(), "test".to_string())),
        Arc::new(Search::new("Test2".to_string(), "test2".to_string())),
        Arc::new(Search::new("Test3".to_string(), "test3".to_string())),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    subito.add_user(String::from("1234")).await?;

    assert_eq!(*notifier_spy.users.lock().await, vec!["1234"]);
    Ok(())
}
