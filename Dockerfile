# Build
FROM rust:1.57.0 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
ENV SQLX_OFFLINE true
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin netflux-service

# Runtime
FROM ubuntu:22.04 AS runtime
WORKDIR /usr/app
COPY --from=builder /app/target/release/netflux-service netflux-service
COPY settings settings
ENV APP_ENVIRONMENT production
ENTRYPOINT ["/usr/app/netflux-service"]
