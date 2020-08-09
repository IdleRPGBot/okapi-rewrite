FROM docker.io/library/alpine:edge AS builder

ENV CXX="g++"
ENV CC="gcc"

WORKDIR /build

RUN apk add --no-cache make cmake harfbuzz-dev openssl-dev curl gcc g++ musl-dev && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY . .

RUN source $HOME/.cargo/env && \
    cargo build --release && \
    strip /build/target/release/okapi

FROM docker.io/library/alpine:edge

WORKDIR /okapi

COPY --from=builder /build/target/release/okapi /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi
