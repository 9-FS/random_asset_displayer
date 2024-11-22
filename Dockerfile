ARG RUST_VERSION="1.82"

FROM rust:$RUST_VERSION as builder
WORKDIR "/app/"
COPY "." "."
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR "/app/"
COPY --from=builder "/app/target/release/random_asset_displayer" "."

CMD ["./random_asset_displayer"]


# MANUAL BUILD:

# build docker image, save in tar, remove image so only tar remains
# docker build -t "9-fs/random_asset_displayer:latest" --no-cache . && docker save "9-fs/random_asset_displayer:latest" > "docker-image.tar" && docker rmi "9-fs/random_asset_displayer:latest"

# on deployment environment load docker image from tar file
# docker load < "/mnt/user/appdata/docker-image.tar"