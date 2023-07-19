#!/usr/bin/env bash

echo "The test will start-up a KV database and populate it with data from the Ethereum blockchain."
set -o pipefail # Will fail script for `grpcurl ... | jq ...` commands

if [ ! -f start-kv-sink.sh ]; then
	echo "Could not find start-kv-sink.sh script to start gRPC server"
fi

if ! type "jq" > /dev/null; then
	echo "Could not find \`jq\` command. See https://jqlang.github.io/jq/ to install."
fi

if ! type "grpcurl" > /dev/null; then
	echo "Could not find \`jq\` command. See https://github.com/fullstorydev/grpcurl to install."
fi

printf "\nStarting gRPC server...\n"
./start-kv-sink.sh > /dev/null 2>&1 &
SERVER_PID=$!
sleep 60s
printf "gRPC server started !\n"

printf "\nTesting \`BlockIdByTime\` queries...\n"
# Correct
if grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2015-07-30T15:26:28Z"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime | jq -e '.id == "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" and .number == "1"' > /dev/null; then
	echo "{\"timestamp\": \"2015-07-30T15:26:28Z\"} -- OK"
else
	echo "{\"timestamp\": \"2015-07-30T15:26:28Z\"} -- FAILED"
	RETURN_CODE=1
fi

if grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2015-07-30"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime | jq -e '.id == "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" and .number == "1"' > /dev/null; then
	echo "{\"timestamp\": \"2015-07-30\"} -- OK"
else
	echo "{\"timestamp\": \"2015-07-30\"} -- FAILED"
	RETURN_CODE=1
fi

# Incorrect
if ! grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2010-07-30"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime 2>/dev/null; then
	echo "{\"timestamp\": \"2010-07-30\"} -- ERROR OK (No data for timestamp)"
else
	echo "{\"timestamp\": \"2010-07-30\"} -- ERROR FAILED (No data for timestamp)"
	RETURN_CODE=1
fi

if ! grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2015-07-30 15:26:28"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime 2>/dev/null; then
	echo "{\"timestamp\": \"2015-07-30 15:26:28\"} -- ERROR OK (Invalid timestamp format)"
else
	echo "{\"timestamp\": \"2015-07-30 15:26:28\"} -- ERROR FAILED (Invalid timestamp format)"
	RETURN_CODE=1
fi

printf "\nTesting \`BlockTimeById\` queries...\n"
# Correct
if grpcurl -plaintext -proto ./proto/service.proto -d '{"number": "1"}' localhost:7878 pinax.service.v1.BlockTime.BlockTimeById | jq -e '.timestamp == "2015-07-30T15:26:28Z"' > /dev/null; then
	echo "{\"number\": \"1\"} -- OK"
else
	echo "{\"number\": \"1\"} -- FAILED"
	RETURN_CODE=1
fi

# Incorrect
if ! grpcurl -plaintext -proto ./proto/service.proto -d '{"number": "-1"}' localhost:7878 pinax.service.v1.BlockTime.BlockTimeById 2>/dev/null; then
	echo "{\"number\": \"-1\"} -- ERROR OK (Invalid number)"
else
	echo "{\"number\": \"-1\"} -- ERROR FAILED (Invalid number)"
	RETURN_CODE=1
fi

printf "\nTesting \`BlockRangeByDate\` queries...\n"
# Correct
if grpcurl -plaintext -proto ./proto/service.proto -d '{"first_date": "2015-07-30T15:26:28Z"}' localhost:7878 pinax.service.v1.BlockTime.BlockRangeByDate | jq -e '.range[0].id == "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" and .range[0].number == "1" and .range[1].id == "ab79f822909750f88dfb9dd0350c1ebe98d5495e9c969cdeb6e0ac993b80175b" and .range[1].number == "6912"' > /dev/null; then
	echo "{\"first_date\": \"2015-07-30T15:26:28Z\"} -- OK"
else
	echo "{\"first_date\": \"2015-07-30T15:26:28Z\"} -- FAILED"
	RETURN_CODE=1
fi

if grpcurl -plaintext -proto ./proto/service.proto -d '{"first_date": "2015-07-30"}' localhost:7878 pinax.service.v1.BlockTime.BlockRangeByDate | jq -e '.range[0].id == "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" and .range[0].number == "1" and .range[1].id == "ab79f822909750f88dfb9dd0350c1ebe98d5495e9c969cdeb6e0ac993b80175b" and .range[1].number == "6912"' > /dev/null; then
	echo "{\"first_date\": \"2015-07-30\"} -- OK"
else
	echo "{\"first_date\": \"2015-07-30\"} -- FAILED"
	RETURN_CODE=1
fi

if grpcurl -plaintext -proto ./proto/service.proto -d '{"first_date": "2015-07-30", "second_date": "2015-08-30"}' localhost:7878 pinax.service.v1.BlockTime.BlockRangeByDate | jq -e '.range[0].id == "88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" and .range[0].number == "1" and .range[1].id == "c76c70bf56e8b040e8fe4a0ecddba98f30ddbaf0b244749b8b4ce6ad3471eb66" and .range[1].number == "162118"' > /dev/null; then
	echo "{\"first_date\": \"2015-07-30\", \"second_date\": \"2015-08-30\"} -- OK"
else
	echo "{\"first_date\": \"2015-07-30\", \"second_date\": \"2015-08-30\"} -- FAILED"
	RETURN_CODE=1
fi

# Incorrect
if ! grpcurl -plaintext -proto ./proto/service.proto -d '{"first_date": "2010-07-30"}' localhost:7878 pinax.service.v1.BlockTime.BlockRangeByDate 2>/dev/null; then
	echo "{\"first_date\": \"2010-07-30\"} -- ERROR OK (No data for timestamp)"
else
	echo "{\"first_date\": \"2010-07-30\"} -- ERROR FAILED (No data for timestamp)"
	RETURN_CODE=1
fi

if ! grpcurl -plaintext -proto ./proto/service.proto -d '{"first_date": "2015-07-30 15:26:28"}' localhost:7878 pinax.service.v1.BlockTime.BlockRangeByDate 2>/dev/null; then
	echo "{\"first_date\": \"2015-07-30 15:26:28\"} -- ERROR OK (Invalid timestamp format)"
else
	echo "{\"first_date\": \"2015-07-30 15:26:28\"} -- ERROR FAILED (Invalid timestamp format)"
	RETURN_CODE=1
fi

pkill -P ${SERVER_PID}
exit $RETURN_CODE