#! /usr/bin/env bash

docker images | grep fakebook-db > /dev/null || $(dirname -- "$0")/build.sh
docker ps | grep fakebook-db > /dev/null && docker stop fakebook-db
docker ps -a | grep fakebook-db > /dev/null && docker rm fakebook-db

# --cap-add NET_ADMIN

(
    trap "kill 0" SIGINT
    docker run \
        -it \
        -p 127.0.0.1:5432:5432 \
        -v fakebook-volume:/var/lib/postgresql/data \
        --name fakebook-db \
        --rm \
        fakebook-db \
        -c log_statement=all
)