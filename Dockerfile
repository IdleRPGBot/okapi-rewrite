FROM registry.fedoraproject.org/fedora-minimal:34 AS builder

WORKDIR /build

RUN microdnf install -y cmake clang make findutils lld && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY Cargo.toml Cargo.lock ./

RUN source $HOME/.cargo/env && \
    mkdir -p src/ && \
    echo "fn main() {println!(\"broken\")}" > src/main.rs && \
    cargo build --release

COPY src src/

RUN set -ex && \
    source $HOME/.cargo/env && \
    find target/release -type f -name "okapi" -exec touch -t 200001010000 {} + && \
    cargo build --release && \
    strip /build/target/release/okapi

FROM registry.fedoraproject.org/fedora-minimal:34

WORKDIR /okapi

COPY --from=builder /build/target/release/okapi /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi
