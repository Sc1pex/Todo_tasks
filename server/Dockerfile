FROM rust:latest

# Install pglib
RUN apt-get install libpq-dev -y

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

CMD ["cargo", "run", "--release"]
