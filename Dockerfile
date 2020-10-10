# Because there is no such thing as proper toolchains
# for cross-compiling to glibc (musl.cc is just superior)
# We ONLY support aarch64 and x86_64 host via linaro

# Rust syntax target, either x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu
ARG RUST_TARGET="x86_64-unknown-linux-gnu"
# glibc target, either x86_64-linux-gnu, aarch64-linux-gnu
ARG GLIBC_TARGET="x86_64-linux-gnu"
# This ONLY works with defaults which is rather annoying
# but better than nothing
# Uses docker's own naming for architectures
# e.g. x86_64 -> amd64, aarch64 -> arm64v8
ARG FINAL_TARGET="amd64"

FROM docker.io/library/fedora:rawhide AS builder
ARG RUST_TARGET
ARG GLIBC_TARGET

RUN curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly -y && \
    source $HOME/.cargo/env && \
    if [ "$RUST_TARGET" != "x86_64-unknown-linux-gnu" ]; then \
        rustup target add $RUST_TARGET; \
        dnf install -y xz gcc clang && \
        curl -L https://developer.arm.com/-/media/Files/downloads/gnu-a/9.2-2019.12/binrel/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu.tar.xz -o /toolchain.tar.xz && \
        tar xf toolchain.tar.xz && \
        ln -s "/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/bin/aarch64-none-linux-gnu-gcc" "/usr/bin/$GLIBC_TARGET-gcc" && \
        ln -s "/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/bin/aarch64-none-linux-gnu-ld" "/usr/bin/$GLIBC_TARGET-ld" && \
        ln -s "/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/bin/aarch64-none-linux-gnu-strip" "/usr/bin/actual-strip" && \
        ln -s "/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/bin/aarch64-none-linux-gnu-g++" "/usr/bin/$GLIBC_TARGET-g++"; \
    else \
        dnf install -y gcc clang lld && \
        ln -s /usr/bin/strip /usr/bin/actual-strip; \
    fi

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo/

RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs
RUN source $HOME/.cargo/env && \
    if [ "$RUST_TARGET" != "x86_64-unknown-linux-gnu" ]; then \
        export CXXFLAGS="--sysroot /gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/aarch64-none-linux-gnu/libc -I/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/aarch64-none-linux-gnu/include/c++/9.2.1 -I/gcc-arm-9.2-2019.12-x86_64-aarch64-none-linux-gnu/aarch64-none-linux-gnu/include/c++/9.2.1/aarch64-none-linux-gnu"; \
    fi && \
    cargo build --release \
        --target="$RUST_TARGET"

RUN rm -f target/$RUST_TARGET/release/deps/okapi_rewrite*

COPY ./src ./src

RUN set -ex && \
    source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET" && \
    actual-strip /build/target/$RUST_TARGET/release/okapi-rewrite && \
    cp /build/target/$RUST_TARGET/release/okapi-rewrite /okapi

FROM docker.io/${FINAL_TARGET}/fedora:rawhide

WORKDIR /okapi

COPY --from=builder /okapi /usr/bin/
COPY assets /okapi/assets

CMD /usr/bin/okapi
