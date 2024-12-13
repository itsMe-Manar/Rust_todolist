# CQRS Rust Example

In this repository I document my experiments with the event-sourcing framework [cqrs-es](https://doc.rust-cqrs.org)

Mostly I follow the example provided in their [documentation](https://doc.rust-cqrs.org). After that I want
to persist the events using a database (probably [postgress](https://crates.io/crates/postgres-es)) and make a simple
web-frontend (probably using [htmx](https://htmx.org))

## Next steps

- [ ] Implement handling of all commands
- [ ] Integrate a persistent data store (with postgres)
- [ ] Build a simple web-based UI

## Backlog

- [ ] Integrate [quickcheck tests](https://github.com/BurntSushi/quickcheck)
- [ ] Integrate [test-containers](https://testcontainers.com/?language=rust)
