#!/usr/bin/env bash

VERSION=$1

if [[ -z "$1" ]] ; then
    echo "Usage: ./scripts/docker-hub-publish.sh VERSION"
    exit 1
fi

docker build . -t zulustation/zulu-node:$1 -t zulustation/zulu-node:latest --build-arg PROFILE=production
docker push zulustation/zulu-node:$1
docker push zulustation/zulu-node:latest
