# Builder stage
FROM rust:1.57.0 AS builder

WORKDIR /app
COPY backend .
RUN cargo build --release

# Frontend builder stage
FROM node:16 AS frontbuilder

WORKDIR /app
COPY frontend/package*.json .
RUN npm install

COPY frontend/tsconfig.json .
COPY frontend/src/ ./src
COPY frontend/public/ ./public

RUN npm run build

# Runtime stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=builder /app/target/release/anisoc anisoc
RUN mkdir target
COPY --from=frontbuilder /app/build target/public
# We need the configuration file at runtime!
COPY backend/configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./anisoc"]