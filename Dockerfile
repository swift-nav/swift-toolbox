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
                        xz-utils \
  && cargo install --force cargo-make taplo-cli

ENV PATH=/usr/local/cargo/bin:/usr/lib/qt6/bin:${PATH}

RUN useradd -u 1000 -ms /bin/bash -G sudo builder

USER builder
WORKDIR /work

ENV HOME=/home/builder
ENV CARGO_HOME=${HOME}/.cargo
ENV XDG_SESSION_TYPE=x11

# Change `cargo make create-dist` to `cargo make run` to build and execute the
# console in-place (using a debug build).  Note that this requires Docker to be
# configured to forward the display to the host, e.g.:
# docker run -it --net=host -v $PWD:/work:rw -v /tmp/.X11-unix:/tmp/.X11-unix -v $HOME/.Xauthority:/home/builder/.Xauthority -e DISPLAY=$DISPLAY -h $HOSTNAME <image hash>
#
# Developers may also wish to use `cargo make build-console` to perform
# the compilation stages without creating the entire redistributable package.
CMD cargo make setup-builder && cargo make create-dist
