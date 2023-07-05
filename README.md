# Kroegen

## TODO

- Implement session based login using this example: https://github.com/SergioBenitez/Rocket/tree/master/examples/cookies


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
```


## Frontend

```bash
cd frontend/
deno task dev
```