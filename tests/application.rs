use std::{error::Error, rc::Rc};

use rs_subito_alert::{
    application::{application_api::ApplicationApi, subito::Subito},
    query_db::search::Search,
};

use crate::test_doubles::{
    notifier::NotifierSpy,
    query::{QueryDbFake, QueryDbSpy},
    scraper::{ScraperFake, ScraperSpy},
};

mod test_doubles;

#[test]
fn test_add_search() {
    let mut query_spy = QueryDbSpy::new();
    let mut scraper = ScraperFake {};
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

    let _ = subito.add_search(String::from("Test"), String::from("test"));

    assert_eq!(
        query_spy.invocations,
        vec![Rc::new(Search::new(
            String::from("Test"),
            String::from("test")
        ))]
    )
}

#[test]
fn test_delete_search() {
    let mut query_spy = QueryDbSpy::new();
    let mut scraper = ScraperFake {};
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

    let _ = subito.delete_search(String::from("Test"));

    assert_eq!(query_spy.deletions, vec![String::from("Test")])
}

#[test]
fn test_list_search() {
    let mut query_spy = QueryDbSpy::new();
    let mut scraper = ScraperFake {};
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_spy, &mut scraper, &mut notifier_spy);

    let _ = subito.list();

    assert_eq!(query_spy.lists, vec![()])
}

#[test]
fn test_scrape() -> Result<(), Box<dyn Error>> {
    let mut scraper_spy = ScraperSpy::new();
    let mut query_fake = QueryDbFake::new();
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

    let _ = subito.scrape();

    assert_eq!(scraper_spy.invocations, 3);
    Ok(())
}

#[test]
fn test_scrape_results() -> Result<(), Box<dyn Error>> {
    let mut scraper_spy = ScraperSpy::new();
    let mut query_fake = QueryDbFake::new();
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

    let results = subito.scrape()?;

    assert_eq!(scraper_spy.invocations, (results.len() as i32));
    Ok(())
}

#[test]
fn test_notification_on_new_items() -> Result<(), Box<dyn Error>> {
    let mut scraper_spy = ScraperSpy::new();
    let mut query_fake = QueryDbFake::new();
    let mut notifier_spy = NotifierSpy::default();
    let mut subito = Subito::new(&mut query_fake, &mut scraper_spy, &mut notifier_spy);

    let results = subito.scrape()?;

    assert_eq!(notifier_spy.invocations, (results.len() as i32) - 2);
    Ok(())
}
