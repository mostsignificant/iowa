########################################################################################################################
# iowa build stage
########################################################################################################################

FROM rust:1.77.0-slim as build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "iowa"

RUN cargo build --target x86_64-unknown-linux-musl --release

########################################################################################################################
# iowa image
########################################################################################################################

FROM scratch

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

COPY --from=build --chown=iowa:iowa ./target/x86_64-unknown-linux-musl/release/iowa /app/iowa

USER iowa:iowa

ENTRYPOINT ["./app/iowa"]
