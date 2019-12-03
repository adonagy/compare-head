FROM rust:latest

COPY . .

RUN cargo install --path .

CMD ["compare-head"]
