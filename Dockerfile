FROM registry.fedoraproject.org/fedora-minimal:32 AS builder

WORKDIR /build

RUN microdnf install -y g++ cmake gcc make harfbuzz-devel openssl-devel && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY Cargo.toml .
COPY Cargo.lock .

RUN source $HOME/.cargo/env && \
    mkdir src && \
    echo "// dummy file" > src/lib.rs && \
    cargo build --release

COPY . .

RUN source $HOME/.cargo/env && \
    cargo build --release && \
    strip /build/target/release/okapi-rewrite

FROM registry.fedoraproject.org/fedora-minimal:32

WORKDIR /okapi

RUN microdnf install -y libstdc++ harfbuzz-devel && \
    microdnf clean all

COPY --from=builder /build/target/release/okapi-rewrite /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi-rewrite
