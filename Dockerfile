ARG RUST_VERSION="1.80"

FROM rust:$RUST_VERSION as builder
WORKDIR "/app/"
COPY "." "."
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR "/app/"
COPY --from=builder "/app/target/release/random_redirector" "."

CMD ["./random_redirector"]


# MANUAL BUILD:

# build docker image, save in tar, remove image so only tar remains, @L to lowercase
# IMAGE_NAME="9-FS/random_redirector:latest" && docker build -t "${IMAGE_NAME@L}" --no-cache . && docker save "${IMAGE_NAME@L}" > "docker-image.tar" && docker rmi "${IMAGE_NAME@L}"

# on deployment environment load docker image from tar file
# docker load < "/mnt/user/appdata/docker-image.tar"