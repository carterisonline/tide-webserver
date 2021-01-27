FROM rust:1.49 as build 
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/actix-webserver
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/actix-webserver /usr/local/bin/actix-webserver

CMD ["actix-webserver"]
