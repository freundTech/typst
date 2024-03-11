FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx
FROM --platform=$BUILDPLATFORM rust:alpine AS build
COPY --from=xx / /

RUN apk add clang lld
COPY . /app
WORKDIR /app
RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    cargo fetch

ARG TARGETPLATFORM
RUN xx-apk add --update musl-dev openssl-dev openssl-libs-static
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    OPENSSL_NO_PKG_CONFIG=1 OPENSSL_STATIC=1 \
    OPENSSL_DIR=$(xx-info is-cross && echo /$(xx-info)/usr/ || echo /usr) \
    xx-cargo build -p typst-cli --release && \
    cp target/$(xx-cargo --print-target-triple)/release/typst target/release/typst && \
    xx-verify target/release/typst

FROM alpine:latest
WORKDIR /root/
COPY --from=build  /app/target/release/typst /bin
