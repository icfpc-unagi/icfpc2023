FROM rust:1.70 AS rust-builder
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
# RUN cargo install wasm-pack  # It was very slow.
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

FROM rust-builder AS vis
COPY Cargo.toml /app/Cargo.toml
COPY vis/Cargo.toml /app/vis/Cargo.toml
WORKDIR /app
RUN mkdir -p ./src ./vis/src \
    && touch ./src/lib.rs ./vis/src/lib.rs \
    && cd ./vis \
    && cargo vendor \
    && { wasm-pack build --target web || true; } \
    && rm -rf ./src ./vis/src
COPY src /app/src
COPY vis/src /app/vis/src
COPY vis/index.html /www/index.html
RUN touch ./src/lib.rs ./vis/src/lib.rs \
    && cd ./vis \
    && wasm-pack build --target web \
    && cp ./pkg/*.js ./pkg/*.wasm /www/

FROM nginx
RUN rm --rf /var/www/html || true
COPY --from=vis /www /usr/share/nginx/html
