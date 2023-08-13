FROM rust:1.71.1 as build-backend

WORKDIR /home/app

COPY backend .

RUN cargo build --release

RUN target/release/export-schemas > schemas.jsonschema

FROM node:20.5.1 as build-frontend

WORKDIR /home/app

COPY frontend .
RUN npm install

COPY --from=build-backend home/app/schemas.jsonschema .
RUN cat schemas.jsonschema | npx json2ts --additionalProperties false > src/models/schemas.ts

RUN npm run build

FROM debian:bookworm-slim

WORKDIR /opt/kroeg

RUN apt-get update \
    && apt-get install -y libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build-backend /home/app/target/release/server server
COPY --from=build-frontend /home/app/build static

COPY backend/Rocket.toml .

CMD ["/opt/kroeg/server"]
