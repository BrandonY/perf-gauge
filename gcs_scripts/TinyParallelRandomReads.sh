#!/bin/bash

export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH
export GRPC_XDS_EXPERIMENTAL_ENABLE_AGGREGATE_AND_LOGICAL_DNS_CLUSTER=true

# Make sure we're using the JSON API and not the XML API.
export GOOGLE_CLOUD_CPP_STORAGE_REST_CONFIG=disable-xml

BUCKET=gcs-grpc-team-perf-testing-us-central1-parallel-random-reads

while :
do

	./target/release/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job ManyTinyParallelReads \
		--concurrency 50 --duration 2m --max_iter 3 --continuous \
		--name gRPC gcs --project gcs-grpc-team-testing \
		--bucket $BUCKET --objects 1G_random_bytes \
		--scenario read-object --random-range-read-max-start 100000000 --random-range-read-max-len 100 \
		--api grpc-directpath

	./target/release/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job ManyTinyParallelReads \
		--concurrency 50 --duration 2m --max_iter 3 --continuous \
		--name gRPC_CFE gcs --project gcs-grpc-team-testing \
		--bucket $BUCKET --objects 1G_random_bytes \
		--scenario read-object --random-range-read-max-start 100000000 --random-range-read-max-len 100 \
		--api grpc-no-directpath

	./target/release/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job ManyTinyParallelReads \
		--concurrency 50 --duration 2m --max_iter 3 --continuous \
		--name JSON gcs --project gcs-grpc-team-testing \
		--bucket $BUCKET --objects 1G_random_bytes \
		--scenario read-object --random-range-read-max-start 100000000 --random-range-read-max-len 100 \
		--api json
done
