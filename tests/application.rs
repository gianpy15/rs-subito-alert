use std::{error::Error, sync::Arc, cell::RefCell};

use rs_subito_alert::{
    application::{application_api::ApplicationApi, subito::Subito},
    query_db::search::Search, scraper::item_result::ItemResult,
};
use tokio::sync::Mutex;

use crate::test_doubles::{
    notifier::NotifierSpy,
    query::QueryDbDouble,
    scraper::{ScraperFake, ScraperSpy, ScraperDouble},
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

    let _ = subito.add_search("Test", "test").await;

    assert_eq!(
        Arc::clone(&query_spy).lock().await.invocations,
        vec![Arc::new(Search::new("Test", "test", None))]
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

    let _ = subito.delete_search("Test").await;

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

    query_fake
        .lock()
        .await
        .set_items(vec![Arc::from("test"), Arc::from("test2")]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test", "test", None)),
        Arc::new(Search::new("Test2", "test2", None)),
        Arc::new(Search::new("Test3", "test3", None)),
    ]);

    let scraper_spy = Arc::new(ScraperSpy::new());
    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let _ = subito.scrape(None).await;

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

    let results = subito.scrape(None).await?;

    assert_eq!(
        *scraper_spy.invocations.lock().await,
        (results.len() as i32)
    );
    Ok(())
}

#[tokio::test]
async fn test_scrape_with_price_filter() -> Result<(), Box<dyn Error>> {
    let mut scraper_fake = ScraperDouble::new();

    scraper_fake.set_results(vec![
        ItemResult::new_from_str("test", "test", None, Some(10), None, None, None),
        ItemResult::new_from_str("test2", "test", None, Some(30), None, None, None),
        ItemResult::new_from_str("test3", "test2", None, Some(10), None, None, None),
        ItemResult::new_from_str("test4", "test2", None, Some(40), None, None, None),
    ]);

    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake
        .lock()
        .await
        .set_items(vec![]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test", "test", Some(20))),
        Arc::new(Search::new("Test2", "test2", None)),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&Arc::new(scraper_fake)),
        Arc::clone(&notifier_spy),
    );

    let results = subito.scrape(Some(true)).await?;

    assert_eq!(
        *notifier_spy.invocations.lock().await,
        vec![
            ItemResult::new_from_str("test", "test", None, Some(10), None, None, None),
            // ItemResult::new_from_str("test", "test_2", None, Some(30), None, None, None),
            ItemResult::new_from_str("test2", "test2", None, Some(10), None, None, None),
            ItemResult::new_from_str("test3", "test2", None, Some(40), None, None, None),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_notification_on_new_items() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake
        .lock()
        .await
        .set_items(vec![Arc::from("test"), Arc::from("test2")]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test", "test", None)),
        Arc::new(Search::new("Test2", "test2", None)),
        Arc::new(Search::new("Test3", "test3", None)),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let results = subito.scrape(Some(true)).await?;

    assert_eq!(
        notifier_spy.invocations.lock().await.len(),
        results.len() - 2
    );
    Ok(())
}

#[tokio::test]
async fn test_new_items_are_added_to_db() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake
        .lock()
        .await
        .set_items(vec![Arc::from("test"), Arc::from("test2")]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test", "test", None)),
        Arc::new(Search::new("Test2", "test2", None)),
        Arc::new(Search::new("Test3", "test3", None)),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    let _results = subito.scrape(Some(false)).await?;

    assert_eq!(*query_fake.lock().await.adds, vec![Arc::from("test3")]);
    Ok(())
}

#[tokio::test]
async fn test_add_user() -> Result<(), Box<dyn Error>> {
    let scraper_spy = Arc::new(ScraperSpy::new());
    let query_fake = Arc::new(Mutex::new(QueryDbDouble::new()));

    query_fake
        .lock()
        .await
        .set_items(vec![Arc::from("test"), Arc::from("test2")]);

    query_fake.lock().await.set_searches(vec![
        Arc::new(Search::new("Test", "test", None)),
        Arc::new(Search::new("Test2", "test2", None)),
        Arc::new(Search::new("Test3", "test3", None)),
    ]);

    let notifier_spy = Arc::new(NotifierSpy::default());
    let subito = Subito::new(
        Arc::clone(&query_fake),
        Arc::clone(&scraper_spy),
        Arc::clone(&notifier_spy),
    );

    subito.add_user("1234").await?;

    assert_eq!(*notifier_spy.users.lock().await, vec!["1234"]);
    Ok(())
}
