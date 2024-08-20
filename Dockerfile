FROM rust:1.79.0 as build-backend

WORKDIR /home/app

COPY backend .

RUN cargo build --release && \
    target/release/export-schemas > schemas.jsonschema

FROM node:22.5 as build-frontend

WORKDIR /home/app

COPY frontend .
RUN npm install

COPY --from=build-backend home/app/schemas.jsonschema .

RUN npx json2ts --additionalProperties false -i schemas.jsonschema -o src/models/schemas.ts && \
    npm run build


FROM debian:12.6-slim

WORKDIR /opt/kroeg

RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5=15.* \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build-backend /home/app/target/release/server server
COPY --from=build-frontend /home/app/build static

COPY backend/Rocket.toml .

CMD ["/opt/kroeg/server"]
