FROM registry.fedoraproject.org/fedora-minimal:32 AS builder

WORKDIR /build

RUN microdnf install -y cmake clang make openssl-devel && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY . .

RUN source $HOME/.cargo/env && \
    cargo build --release && \
    strip /build/target/release/okapi

FROM registry.fedoraproject.org/fedora-minimal:32

WORKDIR /okapi

COPY --from=builder /build/target/release/okapi /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi
