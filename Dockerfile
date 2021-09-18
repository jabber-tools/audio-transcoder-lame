FROM rust:latest

RUN apt-get update && apt-get install nano

WORKDIR /usr/src/audio-transcoder-lame
COPY . .
															  
RUN cd /usr/src/audio-transcoder-lame && cargo build --release
