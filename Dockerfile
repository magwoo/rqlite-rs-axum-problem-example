FROM rust:1.81.0-alpine AS builder
RUN apk add build-base openssl-dev
COPY . .
RUN cargo build -r 
