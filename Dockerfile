FROM registry.fedoraproject.org/fedora-minimal:32 AS builder

WORKDIR /build

RUN microdnf install -y g++ cmake gcc make harfbuzz-devel openssl-devel && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY Cargo.toml .
COPY Cargo.lock .

RUN source $HOME/.cargo/env && \
    mkdir src && \
    echo "fn main() { }" > src/main.rs && \
    echo "fn main() { }" > src/genprofile.rs && \
    echo "fn main() { }" > src/genoverlay.rs && \
    cargo build --release

COPY . .

RUN source $HOME/.cargo/env && \
    touch -t 200001010000 /build/target/release/okapi && \
    touch -t 200001010000 /build/target/release/okapi-helper-genoverlay && \
    touch -t 200001010000 /build/target/release/okapi-helper-genprofile && \
    cargo build --release && \
    strip /build/target/release/okapi && \
    strip /build/target/release/okapi-helper-genoverlay && \
    strip /build/target/release/okapi-helper-genprofile

FROM registry.fedoraproject.org/fedora-minimal:32

WORKDIR /okapi

RUN microdnf install -y libstdc++ harfbuzz-devel && \
    microdnf clean all

COPY --from=builder /build/target/release/okapi /usr/bin/
COPY --from=builder /build/target/release/okapi-helper-genoverlay /usr/bin/
COPY --from=builder /build/target/release/okapi-helper-genprofile /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi
