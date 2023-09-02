# Subito.it Scraper

This is my first project in Rust.

I want to build a tool for scrape Subito.it website based on some custom searches and periodically send telegram notifications for every new entry found.

I'm studying Rust, suggestions and PRs are more than welcome.

# Hexagonal Architecture

Since I've studied recently Hexagonal architecture, I'm trying to apply it to this project. Here's the outline:

## Hexagon
### Flows
- Add new search
- Get all the searches
- Perform the scrape
- Get results
## Ports
- [X] Application
- [X] Notification
- [X] Scraper
- [X] DataBase
- [X] Serialization
## Adapters
- [X] Application Entry Point
- [X] Notification (telegram)
- [X] Scraper Agent
- [X] Query Engine
- [X] Serializer


# TODO

- [X] Better messages for list and scrape results.
- [X] Reset Apllication in cli.
- [ ] Set Scrape interval in cli.
- [ ] Print help message in bot start.
- [ ] User Management for indipendent notifications.