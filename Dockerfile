FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    vim \
    python3 \
    python3-pip \
    golang-go \
    rustc \
    build-essential \
    nodejs \
    npm

ENV GOPATH /go
ENV PATH $GOPATH/bin:/usr/local/go/bin:$PATH

WORKDIR /
