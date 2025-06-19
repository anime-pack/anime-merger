FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /anime-merger
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /anime-merger/target/x86_64-unknown-linux-musl/release/anime-merger /usr/local/bin/anime-merger
ENTRYPOINT ["/usr/local/bin/anime-merger"]
EXPOSE 3000