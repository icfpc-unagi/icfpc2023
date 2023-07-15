FROM mysql:debian
RUN apt-get update && apt-get install -y \
    zstd \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
ENV MYSQL_ROOT_PASSWORD root
COPY ./after_contest/unagi.sql.zst /
RUN zstd -d unagi.sql.zst -o /docker-entrypoint-initdb.d/1.sql
RUN echo "ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'root';" > /docker-entrypoint-initdb.d/2.sql
