#!/bin/bash

export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH
export GRPC_XDS_EXPERIMENTAL_ENABLE_AGGREGATE_AND_LOGICAL_DNS_CLUSTER=true

while :
do

	./target/debug/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job Read100MObject \
		--concurrency 4 --duration 1m --max_iter 10 \
		--name gRPC_CFE gcs --project gcs-grpc-team-testing \
		--bucket gcs-grpc-team-perf-testing-us-central1 --object 100M \
		--api grpc_no_directpath

	./target/debug/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job Read100MObject \
		--concurrency 4 --duration 1m --max_iter 10 \
		--name gRPC gcs --project gcs-grpc-team-testing \
		--bucket gcs-grpc-team-perf-testing-us-central1 --object 100M \
		--api grpc_directpath

	./target/debug/perf-gauge --prometheus 35.202.89.62:9091 --prometheus_job Read100MObject \
		--concurrency 4 --duration 1m --max_iter 10 \
		--name JSON gcs --project gcs-grpc-team-testing \
		--bucket gcs-grpc-team-perf-testing-us-central1 --object 100M \
		--api json
done
