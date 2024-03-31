########################################################################################################################
# iowa build stage
########################################################################################################################

FROM rust:1.77.0-slim as build

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

########################################################################################################################
# iowa image
########################################################################################################################

FROM debian:buster-slim

RUN useradd -ms /bin/bash iowa
USER iowa

COPY --from=build --chown=iowa:iowa ./target/release/iowa /app/iowa

CMD ["./app/iowa"]
