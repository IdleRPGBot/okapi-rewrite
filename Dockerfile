FROM rustlang/rust:nightly-alpine AS builder

WORKDIR /build

RUN sed -i "s:v3.10:edge:g" /etc/apk/repositories && \
    apk upgrade --no-cache && \
    apk add --no-cache musl-dev g++ cmake gcc make

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release

COPY . .

RUN cargo build --release && \
    strip /build/target/release/okapi-rewrite

FROM alpine:edge

WORKDIR /okapi

RUN apk add --no-cache libstdc++

COPY --from=builder /build/target/release/okapi-rewrite /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi-rewrite
