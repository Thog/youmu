FROM rust:alpine

WORKDIR /usr/src/app

COPY . .

RUN apk add --no-cache pkgconfig gcc musl-dev openssl-dev && cargo install --path . && cargo clean && rm -rf /usr/local/rustup && rm -rf /usr/local/cargo/registry && apk del pkgconfig gcc musl-dev openssl-dev

CMD ["youmu"]