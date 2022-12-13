#!/bin/bash

export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH
export GRPC_XDS_EXPERIMENTAL_ENABLE_AGGREGATE_AND_LOGICAL_DNS_CLUSTER=true

# Make sure we're using the JSON API and not the XML API.
export GOOGLE_CLOUD_CPP_STORAGE_REST_CONFIG=disable-xml

JOB="ReadSmallObject"
OBJECT="18_bytes"

DURATION="1m"
CONCURRENCY=1

NAME_GRPC_NO_DIRECTPATH="gRPC_CFE"
NAME_GRPC_DIRECTPATH="gRPC"
NAME_JSON="JSON"
PROJECT="gcs-grpc-team-testing"
BUCKET="gcs-grpc-team-perf-testing-us-central1"
PROMETHEUS_ADDR="34.173.12.152:9091"

./target/debug/perf-gauge --prometheus $PROMETHEUS_ADDR --prometheus_job $JOB \
	--max_iter 10000 \
	--concurrency $CONCURRENCY --duration $DURATION \
	--name $NAME_GRPC_DIRECTPATH gcs --api grpc-directpath \
	--project $PROJECT --bucket $BUCKET --object $OBJECT &

./target/debug/perf-gauge --prometheus $PROMETHEUS_ADDR --prometheus_job $JOB \
	--max_iter 10000 \
	--concurrency $CONCURRENCY --duration $DURATION \
	--name $NAME_GRPC_NO_DIRECTPATH gcs --api grpc-no-directpath \
	--project $PROJECT --bucket $BUCKET --object $OBJECT &

./target/debug/perf-gauge --prometheus $PROMETHEUS_ADDR --prometheus_job $JOB \
	--max_iter 10000 \
	--concurrency $CONCURRENCY --duration $DURATION \
	--name $NAME_JSON gcs --api json \
	--project $PROJECT --bucket $BUCKET --object $OBJECT &
