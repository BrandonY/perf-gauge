#!/bin/bash

OBJECT=10_bytes.txt
TEST_NAME=10-byte-object-read

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

TEST_NAME="rls_disabled_10-byte-object-read"
UNIVERSE="prod"
BUCKET="rls-perf-pr_ir_lcl_rg_rls_read-d_16mb_json_us_central1"

./target/release/perf-gauge --prometheus $PROMETHEUS \
	--prometheus_label=location=${LOCATION},api=JSON,universe=${UNIVERSE} --name $TEST_NAME \
	--concurrency 1 --duration 1m --max_iter 1000000 --continuous \
	gcs --project gcs-grpc-team-testing --universe=${UNIVERSE} \
	--bucket $BUCKET --objects $OBJECT \
	--scenario read-object \
	--api json