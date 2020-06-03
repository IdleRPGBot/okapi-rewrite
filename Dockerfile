FROM rustlang/rust:nightly-alpine AS builder

WORKDIR /build

RUN apk upgrade --no-cache && apk add --no-cache musl-dev

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release

COPY . .

RUN cargo build --release

FROM alpine:edge

WORKDIR /okapi

COPY --from=builder /build/target/release/okapi-rewrite /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi-rewrite
