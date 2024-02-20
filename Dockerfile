FROM rust:1.70.0-slim as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
RUN apt install protobuf-compiler -y

COPY ./ /usr/src/rinha/
WORKDIR /usr/src/rinha
RUN cargo install protobuf-codegen && PATH="$HOME/.cargo/bin:$PATH"
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.16.0 AS runtime

ARG USERNAME=rinha
ARG USER_UID=1000
ARG USER_GID=$USER_UID

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/src/rinha/target/x86_64-unknown-linux-musl/release/rinha /usr/local/bin
COPY --from=builder /usr/src/rinha/config /etc/rinha/config

RUN addgroup $USER_GID
RUN adduser --shell /sbin/nologin --disabled-password \
    --no-create-home --uid $USER_UID --ingroup $USER_GID $USERNAME

USER ${USERNAME}

EXPOSE 7080

CMD ["rinha", "-c", "/etc/rinha/config/app.yaml"]