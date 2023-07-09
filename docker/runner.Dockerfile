FROM rust:1.70 AS rust-builder
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update -qy && apt-get install -qy apt-transport-https ca-certificates gnupg curl
RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] http://packages.cloud.google.com/apt cloud-sdk main" \
    | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list \
    && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg \
    | apt-key --keyring /usr/share/keyrings/cloud.google.gpg  add - \
    && apt-get update -y && apt-get install google-cloud-sdk -y
COPY ./secrets/service_account.json /service_account.json
RUN gcloud auth activate-service-account icfpc2023@icfpc-primary.iam.gserviceaccount.com \
        --key-file=/service_account.json \
    && gcloud config set project icfpc-primary
COPY ./scripts/exec.sh /usr/local/bin/exec.sh
RUN chmod +x /usr/local/bin/exec.sh
COPY ./bin/gcsrun /usr/local/bin/gcsrun
RUN chmod +x /usr/local/bin/gcsrun

WORKDIR /work

ENV RUST_BACKTRACE=1
ARG UNAGI_PASSWORD
ENV UNAGI_PASSWORD ${UNAGI_PASSWORD}

# ENTRYPOINT ["/usr/local/bin/exec.sh"]
