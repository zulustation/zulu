# Based from https://github.com/paritytech/substrate/blob/master/.maintain/Dockerfile

FROM phusion/baseimage:bionic-1.0.0 as builder
LABEL maintainer="hi@zeitgeit.pm"
LABEL description="This is the build stage for the Zulu node. Here is created the binary."

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
ARG FEATURES=default
WORKDIR /zulu

COPY . /zulu

RUN apt-get update && \
    apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
    apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH="$PATH:$HOME/.cargo/bin" && \
    rustup toolchain install nightly-2022-04-13 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-04-13 && \
    rustup default stable && \
    cargo build --profile "$PROFILE" --features "$FEATURES"

# ==== SECOND STAGE ====

FROM phusion/baseimage:bionic-1.0.0
LABEL maintainer="hi@zulu.pm"
LABEL description="This is the 2nd stage: a very small image where we copy the Zeigeist node binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/* && \
    mv /tmp/ca-certificates /usr/share/ && \
    useradd -m -u 1000 -U -s /bin/sh -d /zulu zulu

COPY --from=builder /zulu/target/$PROFILE/zulu /usr/local/bin

# checks
RUN ldd /usr/local/bin/zulu && \
    /usr/local/bin/zulu --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
    rm -rf /usr/bin /usr/sbin /usr/share/man

USER zulu
EXPOSE 30333 9933 9944

RUN mkdir /zulu/data

VOLUME ["/zulu/data"]

ENTRYPOINT ["/usr/local/bin/zulu"]
