FROM mysql:debian
RUN apt-get update && apt-get install -y \
    zstd \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
ENV MYSQL_ROOT_PASSWORD root
COPY ./after_contest/unagi.sql.zst /docker-entrypoint-initdb.d/1.sql.zst
ARG UNAGI_PASSWORD
RUN echo "ALTER USER 'root'@'%' IDENTIFIED WITH mysql_native_password BY '${UNAGI_PASSWORD}';" > /docker-entrypoint-initdb.d/2.sql
