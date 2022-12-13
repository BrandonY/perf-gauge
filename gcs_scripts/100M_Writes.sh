#!/bin/bash


OBJECT="writes/tmp/obj"
TEST_NAME=100m-object-write

export LD_LIBRARY_PATH=./libs:$LD_LIBRARY_PATH
export GRPC_XDS_EXPERIMENTAL_ENABLE_AGGREGATE_AND_LOGICAL_DNS_CLUSTER=true
export GOOGLE_CLOUD_CPP_STORAGE_REST_CONFIG=disable-xml

PROMETHEUS=34.173.12.152:9091
PROJECT=gcs-grpc-team-testing
LOCATION=$(curl http://metadata.google.internal/computeMetadata/v1/instance/zone -H Metadata-Flavor:Google | cut '-d/' -f4 | cut -d- -f1-2)
HOSTNAME=$(uname -n)
if grep -q "preprod" <<< "$HOSTNAME"; then
	UNIVERSE="preprod"
	BUCKET_PREFIX="gcs-grpc-team-preprod-perf"
else
	UNIVERSE="prod"
	BUCKET_PREFIX="gcs-grpc-team-perf-testing"
fi
BUCKET="${BUCKET_PREFIX}-${LOCATION}-${TEST_NAME}"

while :
do

	./target/release/perf-gauge --prometheus $PROMETHEUS \
		--prometheus_label=location=${LOCATION},api=gRPC,universe=${UNIVERSE} --name $TEST_NAME \
		--concurrency 4 --duration 1m --max_iter 3 --continuous \
		gcs --project gcs-grpc-team-testing --universe=${UNIVERSE} \
		--bucket $BUCKET --objects $OBJECT \
		--scenario write-object --object-size 100000000 \
		--api grpc-directpath

	# Disabling CFE metrics for now.
	# ./target/release/perf-gauge --prometheus $PROMETHEUS \
	# 	--prometheus_label=location=${LOCATION},api=gRPC_CFE,universe=${UNIVERSE} --name $TEST_NAME \
	# 	--concurrency 4 --duration 1m --max_iter 3 --continuous \
	# 	gcs --project gcs-grpc-team-testing --universe=${UNIVERSE} \
	# 	--bucket $BUCKET --objects $OBJECT \
	# 	--scenario write-object --object-size 100000000 \
	# 	--api grpc-no-directpath &

	./target/release/perf-gauge --prometheus $PROMETHEUS \
		--prometheus_label=location=${LOCATION},api=JSON,universe=${UNIVERSE} --name $TEST_NAME \
		--concurrency 4 --duration 1m --max_iter 3 --continuous \
		gcs --project gcs-grpc-team-testing --universe=${UNIVERSE} \
		--bucket $BUCKET --objects $OBJECT \
		--scenario write-object --object-size 100000000 \
		--api json &
done
