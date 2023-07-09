FROM ubuntu:22.04

ARG UNAGI_PASSWORD
ENV UNAGI_PASSWORD ${UNAGI_PASSWORD}

ADD bin/apt-install /usr/local/bin/apt-install
RUN apt-install openssl make jq curl ca-certificates
