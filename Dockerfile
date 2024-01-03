FROM rust:1.73.0 as builder

WORKDIR /app

COPY Cargo.lock Cargo.toml ./

RUN cargo new --lib everword-lib
RUN cargo new --bin backend

# Build everword-lib

WORKDIR /app/everword-lib

COPY ./everword-lib/Cargo.toml .

RUN cargo build --release

RUN rm ./src/*.rs
COPY ./everword-lib/src ./src
RUN touch src/lib.rs
RUN cargo build --release

# Build backend

WORKDIR /app
RUN rm -r backend
RUN cargo new --bin backend
WORKDIR /app/backend
COPY ./backend/Cargo.toml .
RUN cargo build --release

RUN rm ./src/*.rs
COPY ./backend/src ./src
RUN touch src/main.rs

RUN cargo build --release

FROM debian:bookworm as runtime

COPY --from=builder /app/target/release/backend /

RUN apt-get update && apt install -y openssl

EXPOSE 8080

CMD [ "/backend" ]