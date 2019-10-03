FROM rust:latest as cargo-build

WORKDIR /usr/src/app

COPY Cargo.* ./

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build -p ${app} --release

RUN rm -f target/release/deps/${app}*

COPY . .

RUN cargo build --release

# Runtime

FROM frolvlad/alpine-glibc

ARG app

COPY --from=cargo-build /usr/src/app/target/release/quotes-exporter /usr/local/bin/quotes-exporter

CMD ["quotes-exporter"]
