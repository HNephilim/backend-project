FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching layer
RUN cargo chef cook --release --recipe-path recipe.json

# Build app
# Copy all files from our working environment to our Docker image
COPY . .

#setup env vars
ENV SQLX_OFFLINE true

RUN cargo build --release --bin zero2prod



FROM debian:bullseye-slim AS runtime
WORKDIR /app

#Update the environment and installing OpenSSL that is need for a dependency
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    #Clean
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

#Copy the compiled binary from the builder environment
#to our runtime environment

COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime
COPY configuration configuration

# Set env vars
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
