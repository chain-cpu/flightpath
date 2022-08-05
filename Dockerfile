FROM rust:1.60.0

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/flightpath"]
