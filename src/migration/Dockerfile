FROM rust:latest

RUN apt-get install -y libpq-dev

WORKDIR /app

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .

