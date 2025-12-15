FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest AS final

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/todos .

RUN chmod +x todos

CMD ["/app/todos"]