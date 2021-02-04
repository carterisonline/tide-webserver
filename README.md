## About tide-webserver:
This project allows me to run a small webserver in less than 1 MiB of ram:
- Similar to static webservers, but loads `index.html` into memory and never reads from disk after that, allowing direct transmission from memory with minimal latancy.
- Supports SSL encryption (though, I'm only using a locally-generated keyfile until I actually get my own domain name.)
- Automatically builds and serves HTML files using [Parcel](https://parceljs.org/)
- Advanced console & verbose logging
- Originally based on the [actix-webserver](https://github.com/carterisonline/actix-webserver) project

## How to run:

- Make sure you're in the current `tide-webserver` directory, then run `./tide-webserver`
- FOR DEVELOPMENT: `cargo run` or `cargo run --release` (for optimized builds)

### Workspace Variables:
- `SSL` (Defaults to `"false"`) - Whether or not to enable SSL encryption
- `WORKDIR` (Defaults to `$PWD`) - The working directory, containing the `web`, `keys`, and (during build) `dist` directories.