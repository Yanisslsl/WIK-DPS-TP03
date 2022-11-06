FROM rust:1-slim

RUN USER=root cargo new --bin course_1

WORKDIR /course_1

COPY ./Cargo.lock ./Cargo.lock

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

RUN rm src/*.rs

COPY ./src ./src

ENV PING_LISTEN_PORT=3008

RUN rm ./target/release/deps/course_1*

RUN cargo install --path .

CMD ["course_1"]







