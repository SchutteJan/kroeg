# Kroegen
Web app to keep track of all _kroegen_ you've been to.

## TODO

MVP:
- Implement session based login using this example: https://github.com/SergioBenitez/Rocket/tree/master/examples/cookies
  - [x] Implement rocket logic for keeping track of sessions and putting routes behind login
  - [x] Implement db models for user management
  - [x] Implement simple RBAC to differentiate admins and users
  - [ ] Frontend login/logout pages
- CSRF on all POST requests
- Allow users to keep track of visited bars
- Allow users to search for a bar (frontend only for now?)

Improvements:
- Use `Forms` for data input instead of `Json<T>` in order to use the `FromForm` macros and validations
- Add "updated_at", "created_at" fields to all tables
- Factor out database queries from the routes into a separate modules
- Use https://crates.io/crates/rocket_okapi to generate OpenAPI spec
- See if switching to https://github.com/launchbadge/sqlx is nice because managing diesel orm types is a pain

Reading:
- Learn from this example repo: https://github.com/TaeyoonKwon/rust-rocket-sample

## System requirements

- [postgresql-libs](https://archlinux.org/packages/extra/x86_64/postgresql-libs/)

## Pre-commit
Install pre-commit hooks:
```bash
pre-commit install
```

The Rust formatter `rustfmt` has unstable features enabled, therefore the nightly toolchain is required to run it.
```bash
rustup toolchain install nightly
```

## Backend

Rust webserver:
```bash
cd backend/
cargo run
```

Diesel ORM:
```bash
cargo install diesel_cli --no-default-features --features postgres
export DATABASE_URL=postgres://postgres:example@localhost/webapp
diesel setup  # Initialize the database in postgres
```


## Frontend

Use the `export-schemas` binary to generate JsonSchema types for use in frontend:
```bash
cd backend/
cargo build --bin export-schemas
```

Build the frontend
```bash
cd frontend/
npm run dev
```
