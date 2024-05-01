FROM rust:buster as builder
RUN apt upgrade && apt update && apt install -y build-essential libssl-dev openssl sqlite3
RUN cargo install sqlx-cli
WORKDIR /workspace

COPY prepare.sql ./
ENV DATABASE_URL=sqlite:./skytrash.db
RUN sqlite3 skytrash.db < prepare.sql

# build dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release

# build project
RUN rm src/main.rs
COPY src ./src/
RUN touch src/main.rs
RUN cargo build --release

FROM debian:buster as runner
WORKDIR /workspace
ENV DATABASE_URL=sqlite:./skytrash.db
COPY --from=builder /workspace/target/release/server server
COPY --from=builder /workspace/skytrash.db skytrash.db
CMD ["./server"]
