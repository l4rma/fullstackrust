FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/fullstackrust
COPY . .

RUN cd client && trunk build --release 
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/fullstackrust/target/release/server /usr/local/bin/server
COPY --from=build /usr/src/fullstackrust/client/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["server"]
