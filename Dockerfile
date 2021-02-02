FROM rust:1.49 as build 
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/tide-webserver
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/tide-webserver /usr/local/bin/tide-webserver

CMD ["tide-webserver"]
