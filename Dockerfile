FROM registry.fedoraproject.org/fedora-minimal:32 AS builder

WORKDIR /build

RUN microdnf install -y g++ cmake gcc clang make harfbuzz-devel openssl-devel && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y

COPY . .

RUN source $HOME/.cargo/env && \
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
