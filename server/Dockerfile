FROM rust:1.77 AS builder
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM debian:stable-slim AS runner
WORKDIR /app
COPY --from=builder /app/target/release/server ./
ENTRYPOINT ["./server"]