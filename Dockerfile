FROM node:22-alpine AS frontend
WORKDIR /src/frontend
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM rust:1.85-alpine AS rust
WORKDIR /src/rust
COPY lacq/Cargo.toml lacq/Cargo.lock ./
COPY lacq/src ./src
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=rust /src/rust/lacq/target/release/lacq /app/lacq
COPY --from=frontend /src/frontend/dist /app/dist
EXPOSE 8080
CMD ["./lacq"]