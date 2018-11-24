FROM ocaml/opam2-staging:alpine as ocaml
WORKDIR /usr/src/app
RUN opam install -y depext
RUN opam depext -y conf-gmp conf-m4 conf-perl
RUN opam install -y yices2
RUN opam install -y qbf
RUN opam pin add touist -y --dev-repo
RUN sudo cp $(opam config var touist:bin)/touist /usr/src/app

# TEMPORARY: as soon as
# https://github.com/SergioBenitez/Rocket/issues/660 is solved,
# replace with 'nightly'
FROM clux/muslrust:latest as rust
# Note: we use a MUSL image so that we get a binary with no dynamically-linked
# binaries so that we can ship using 'FROM scratch'.

WORKDIR /usr/src/app

# IMPORTANT: if the build fails because of some error more or less
# linked to the fact we are using the nightly: first, try to build
# without cache so that the latest rustc compiler is downloaded.
#     docker build . --no-cache

# These steps are here only in order to cache the dependencies in docker.
# See: https://stackoverflow.com/a/39662015/3808537
# Note that 'cargo install' seems to re-build everything; I run 'cargo build'
# instead.
COPY Cargo.toml Cargo.lock dummy.rs Rocket.toml /usr/src/app/
RUN cargo build --lib --release

# Now, copy and build our own code
COPY . /usr/src/app/
RUN cargo build --release

# We don't need anything from above except for the binary produced.
# Using multi-layer will give us a much smaller image. At first, I wanted to
# use the 'FROM scratch' (with statically-compiled binary using muslrust) but
# then I realized that the 'touist' binary isn't stand-alone and is dynamically
# linked to glic. So I switched over to alpine.
FROM alpine:latest
COPY --from=rust /usr/src/app/target/x86_64-unknown-linux-musl/release/touist-server /touist-server
COPY --from=ocaml /usr/src/app/touist /external/

# LOG: critical | normal | debug
ENV PORT=8080  \
    BASE=/     \
    ENV=dev    \
    LOG=normal \
    ADDRESS=0.0.0.0
# ROCKET_ADDRESS must be set to '0.0.0.0' Otherwise, if set to 'localhost'
# or something else, the host won't be able to reach the rocket server for
# some reason.


CMD ["sh", "-c", "ROCKET_PORT=${PORT} ROCKET_ENV=${ENV} ROCKET_BASE=${BASE} ROCKET_ADDRESS=${ADDRESS} ROCKET_LOG=${LOG} /touist-server"]
