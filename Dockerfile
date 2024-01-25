FROM rust:1.70

WORKDIR /usr/src/minicode
COPY . .

RUN cargo install --path .
