# Kroegen

## TODO

- Implement session based login using this example: https://github.com/SergioBenitez/Rocket/tree/master/examples/cookies

## System requirements

- [postgresql-libs](https://archlinux.org/packages/extra/x86_64/postgresql-libs/)

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
