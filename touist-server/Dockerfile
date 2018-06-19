# TEMPORARY: as soon as
# https://github.com/SergioBenitez/Rocket/issues/660 is solved,
# replace with 'nightly'
FROM clux/muslrust:nightly-2018-06-09 as rust
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
COPY . /usr/src/app
RUN cargo build --release

# We don't need anything from above except for the binary produced.
# Using multi-layer will give us a much smaller image.
FROM scratch
COPY --from=rust /usr/src/app/target/x86_64-unknown-linux-musl/release/touist-server /touist-server
ENV PORT=8080
ENV ROCKET_PORT=$PORT
CMD ["/touist-server"]