FROM lscr.io/linuxserver/piper:latest AS piper

FROM rust:1.89-bookworm

ENV DEBIAN_FRONTEND=nointeractive

RUN apt-get update && apt-get install -y \
	ffmpeg \
	make \
	&& rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-binstall --locked
RUN cargo binstall trunk --locked
RUN cargo binstall cargo-watch
RUN rustup target add wasm32-unknown-unknown

COPY --from=piper /root/ /usr/local/piper

WORKDIR /app
COPY . .

# frontend
EXPOSE 8080

CMD ["make", "__dev_container"]
