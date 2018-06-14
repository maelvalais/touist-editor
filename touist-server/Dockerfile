FROM rust:latest

# IMPORTANT: if the build fails because of some error more or less
# linked to the fact we are using the nightly: first, try to build
# without cache so that the latest rustc compiler is downloaded.
#     docker build . --no-cache
RUN rustup default nightly
WORKDIR /usr/src/app

# These steps are here only in order to cache the dependencies in docker.
# See: https://stackoverflow.com/a/39662015/3808537
# Note that 'cargo install' seems to re-build everything; I run 'cargo build'
# instead.
COPY Cargo.toml Cargo.lock dummy.rs Rocket.toml /usr/src/app/
RUN cargo build --lib


# Now, copy and build our own code
COPY . /usr/src/app
RUN cargo build

CMD cargo run
