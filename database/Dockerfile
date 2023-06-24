FROM postgres:15-alpine AS builder
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
COPY ./init /docker-entrypoint-initdb.d/
RUN docker-entrypoint.sh postgres

FROM postgres:15-alpine
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
COPY --chown=postgres:postgres --from=builder /var/lib/postgresql/data /var/lib/postgresql/data
EXPOSE 5432