FROM rust:1.88.0-slim

RUN apt-get update \
  && apt-get install -y cmake \
                        libclang-dev \
                        capnproto \
                        zstd \
                        imagemagick \
                        fonts-freefont-otf \
                        git \
                        qt6-declarative-dev-tools \
                        pkgconf \
                        libssl-dev \
                        g++ \
                        libnss3 \
                        libasound2 \
                        libxkbfile1 \
  && cargo install --force cargo-make taplo-cli

ENV PATH=/usr/local/cargo/bin:/usr/lib/qt6/bin:${PATH}

RUN useradd -u 1000 -ms /bin/bash -G sudo builder

USER builder
WORKDIR /work

ENV HOME=/home/builder
ENV CARGO_HOME=${HOME}/.cargo
ENV XDG_SESSION_TYPE=xcb

# change `build-console` to `run` to execute without fully compiling
CMD cargo make setup-builder; cargo make build-console
