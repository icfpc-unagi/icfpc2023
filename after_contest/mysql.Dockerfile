FROM mysql
COPY ./after_contest/unagi.sql.zst /docker-entrypoint-initdb.d/1.sql.zst
ENV MYSQL_ROOT_PASSWORD root
ARG UNAGI_PASSWORD
RUN echo "ALTER USER 'root'@'%' IDENTIFIED WITH mysql_native_password BY '${UNAGI_PASSWORD}';" > /docker-entrypoint-initdb.d/2.sql
