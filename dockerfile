FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

ENV USER=ishigaki
RUN cargo init .

RUN cargo install --path .

CMD ["myapp"]