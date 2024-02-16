FROM rust:alpine3.18 AS planner
WORKDIR /plan

RUN apk update && \
	apk upgrade --no-cache && \
	apk add musl-dev

RUN cargo install cargo-chef

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json


FROM rust:alpine3.18 AS builder
WORKDIR /build

RUN apk update && \
	apk upgrade --no-cache && \
	apk add musl-dev, libressl, pkg-config

RUN cargo install cargo-chef

COPY --from=planner /plan/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release


FROM alpine:3.18 AS runtime
WORKDIR /var/app

COPY --from=builder /build/target/release/lerpz_backend ./

RUN addgroup -S server && \
	adduser -S lerpz_backend -G server && \
	chown -R lerpz_backend:server /var/www/app

USER lerpz_backend

ENTRYPOINT ["/var/app/lerpz_backend"]