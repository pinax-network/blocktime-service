# Blocktime Query Service

A gRPC service for querying a block id/number from a timestamp and vice-versa.

An example use case can be for converting a date range (e.g. 2018-06-01 to 2018-07-01) to a block range that can be used as input for other substreams.

## Setup

Make sure to have the [`substreams-sink-kv`](https://github.com/streamingfast/substreams-sink-kv) tool installed and available in your `$PATH`.

**Populate the kv-store with data from the remote endpoint**
```bash
./inject.sh <endpoint-url>
```

You can install the [`kvdb`](https://github.com/streamingfast/kvdb) tool to inspect the data stored in the *kv-store* locally:
```bash
go install github.com/streamingfast/kvdb/cmd/kvdb@develop
```

**To inspect the *Timestamp > BlockId* data**
```bash
kvdb read prefix kblock.timestamp --dsn "badger3://$(pwd)/badger_data.db" --decoder="proto://./proto/service.proto@pinax.service.v1.BlockId"
```

**To inspect the *BlockId > Timestamp* data**
```bash
kvdb read prefix kblock.number --dsn "badger3://$(pwd)/badger_data.db" --decoder="proto://./proto/service.proto@pinax.service.v1.BlockTimestamp"
```

## Running the service

**Serve the data from localhost through badgerDB**
```bash
./serve.sh
```

**Service query examples (make sure the `serve` script is running)**
```bash
$ grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2018-06-09T11:57:19Z"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime
{
  "id": "0000006492871283c47f6ef57b00cf534628eb818c34deb87ea68a3557254c6b",
  "number": "100"
}
```

```bash
$ grpcurl -plaintext -proto ./proto/service.proto -d '{"number": "100"}' localhost:7878 pinax.service.v1.BlockTime.BlockTimeById                                                                                                        
{
  "timestamp": "2018-06-09T11:57:19Z"
}
```

**Query by date for *Timestamp > BlockId***
```bash
$ grpcurl -plaintext -proto ./proto/service.proto -d '{"timestamp": "2018-06-09"}' localhost:7878 pinax.service.v1.BlockTime.BlockIdByTime
{
  "id": "00000003d93442ea55d07be4d515700e2b9737c1f485e8a13ebb3550c1a8bb44",
  "number": "3"
}
```

Note that if you expect the block number to match with the start of the day (e.g. the timestamp to be exactly 2018-06-09T00:00:00Z), it might not always be the case.

What you're guaranteed is to get the first known block in the KV store for that particular date. You can check the actual timestamp with the following command.

```bash
$ kvdb read prefix kblock.timestamp:2018-06-09 --dsn "badger3://$(pwd)/badger_data.db" --decoder="proto://./proto/service.proto@pinax.service.v1.BlockId" --limit 1
2023-07-12T13:21:27.121-0400 INFO (kvdb) setting up store {"dsn": "badger3:///home/user/Documents/blocktime-service/badger_data.db"}
2023-07-12T13:21:27.759-0400 INFO (kvdb) store prefix {"prefix": "kblock.timestamp:2018-06-09", "limit": 1}
keys with prefix: kblock.timestamp:2018-06-09
kblock.timestamp:2018-06-09T11:56:30.500Z       ->      {"id":"00000003d93442ea55d07be4d515700e2b9737c1f485e8a13ebb3550c1a8bb44","number":"3"}

Found 1 keys
```

Here the timestamp (`kblock.timestamp:2018-06-09T11:56:30.500Z`) is actually corresponding to the middle of the day as it's the first known block stored for that day. 