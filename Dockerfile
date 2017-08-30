FROM pitkley/rust:stable

LABEL maintainer=maxim.vorobjov@gmail.com

RUN apt-get update
RUN apt-get install pkg-config libssl-dev -y

RUN mkdir -p /rust/app
WORKDIR /rust/app
COPY . /rust/app

RUN cargo build --release

CMD cargo run --release