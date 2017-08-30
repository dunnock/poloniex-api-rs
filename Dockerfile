FROM pitkley/rust:stable as build

LABEL maintainer=maxim.vorobjov@gmail.com

RUN apt-get update
RUN apt-get install pkg-config libssl-dev -y

RUN mkdir -p /rust/app
WORKDIR /rust/app
COPY . /rust/app

RUN cargo build --release


FROM debian:jessie
RUN mkdir -p /rust/app
WORKDIR /rust/app
COPY --from=build /usr/lib/ssl/ /usr/lib/ssl/
COPY --from=build /rust/app/target/release/poloniex .
CMD ./poloniex