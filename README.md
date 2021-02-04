## About tide-webserver:
This project allows me to run a small webserver in less than 1 MiB of ram:
- Similar to static webservers, but loads `index.html` into memory and never reads from disk after that, allowing direct transmission from memory with minimal latancy.
- Supports SSL encryption (though, I'm only using a locally-generated keyfile until I actually get my own domain name.)
- Automatically builds and serves HTML files using [Parcel](https://parceljs.org/)
- Advnaced console & verbose logging
- Originally based on the [actix-webserver](https://github.com/carterisonline/actix-webserver) project

## How to run:

`WORKDIR=~/documents/tide-webserver cargo run`

-- OR --

`WORKDIR=~/documents/tide-webserver ./tide-webserver` (after building)

- `WORKDIR` needs to contain the `keys` and `web` directories in order to access the SSL keys and `index.html`.
