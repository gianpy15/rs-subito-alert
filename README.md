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
- [ ] Application
- [ ] Notification
- [X] Scraper
- [X] DataBase
- [ ] Serialization
## Adapters
- [ ] Application Entry Point
- [ ] Notification (telegram)
- [X] Scraper Agent
- [X] Query Engine
- [ ] Serializer


# TODO

- [ ] Better messages for list and scrape results
- [ ] Reset Apllication in cli
- [ ] Set Scrape interval in cli
- [ ] Print help message in bot start