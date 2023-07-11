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

**To inspect the Timestamp > BlockId data**
```bash
kvdb read prefix kblock.timestamp --dsn "badger3://$(pwd)/badger_data.db" --decoder="proto://./proto/service.proto@pinax.service.v1.BlockId"
```

**To inspect the BlockId > Timestamp data**
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