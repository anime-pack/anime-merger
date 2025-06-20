FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /anime-merger
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

# Image labels
LABEL org.opencontainers.image.title="anime-merger"
LABEL org.opencontainers.image.vendor="Anime Pack"
LABEL org.opencontainers.image.description="A anime data API providing info from various APIs"
LABEL org.opencontainers.image.source=https://github.com/anime-pack/anime-merger
LABEL org.opencontainers.image.licenses=Apache-2.0

COPY --from=builder /anime-merger/target/x86_64-unknown-linux-musl/release/anime-merger /usr/local/bin/anime-merger
ENTRYPOINT ["/usr/local/bin/anime-merger"]
EXPOSE 3000