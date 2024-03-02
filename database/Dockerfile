FROM postgres:15-alpine
ENV POSTGRES_DB postgres
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
COPY ./init /docker-entrypoint-initdb.d/
EXPOSE 5432
# RUN apk --update add iproute2
