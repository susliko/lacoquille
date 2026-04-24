FROM node:22-alpine AS frontend
WORKDIR /src/frontend
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM rust:1.86-bookworm AS rust
WORKDIR /src/rust
COPY lacq/ .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust /src/rust/target/release/lacq /app/lacq
COPY --from=frontend /src/frontend/dist /app/dist
EXPOSE 8080
CMD ["./lacq"]
