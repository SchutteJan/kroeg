# Kroegen
Web app to keep track of all _kroegen_ you've been to.

[kroeg.jan.tf](https://kroeg.jan.tf)

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

In order to run the webserver, you will need to have the postgres database setup and have the imgproxy service running.
Both can be started using docker-compose:
```bash
docker compose up db imgproxy -d
```

You will need to manually create the `webapp` database in Postgres the first time:
```bash
docker compose exec db psql -U postgres

postgres=# CREATE DATABASE webapp;
```

Diesel cli is needed to generate new database migrations:
```bash
cargo install diesel_cli --no-default-features --features postgres
export DATABASE_URL=postgres://postgres:example@localhost/webapp
diesel setup
```

All database migrations are applied automatically on webserver start.
```bash
cd backend/
cargo run
```

Next up is creating the local default user, the request to create this user can be found in [backend/requests/session.http](./backend/requests/session.http).

Then you'll have to upgrade this account to an admin:
```bash
docker compose exec db psql -U postgres -d webapp
webapp=# UPDATE users SET role = 'admin' WHERE id = '1';
```

## Frontend

Use the `export-schemas` binary to generate JsonSchema types for use in frontend:
```bash
cd backend/
cargo build --bin export-schemas
```

Build the frontend:
```bash
cd frontend/
npm run schemas
npm run build
```

> [!NOTE]
> There are some pending improvements for the local setup, like making the dev server usable using `npm run dev`
> and not hardcoding the api url in [frontend/src/api/base.ts](./frontend/src/api/base.ts)


## Future work

### Features
- Password recovery
- Improve data pipeline, there are still some kroegen missing due to them being marked as restaurants in the dataset from Gemeente Amsterdam.
- Search for a bar, simple filters for finding a bar
- CSRF on POST requests
- Show errors in the UI using toast notifications

### Issues
- The api never returns a 404, it will always fall back to the 200.html page
  - Example: `curl http://localhost:8080/session/doesnotexist`

### General Improvements
- Improve local setup, setting up initial dataset and a working frontend development server
- Use `Forms` for data input instead of `Json<T>` in order to use the `FromForm` request guards
- Add "updated_at", "created_at" fields to all tables
- Use [rocket_okapi](https://crates.io/crates/rocket_okapi) to generate OpenAPI spec of the api
