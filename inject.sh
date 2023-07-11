#!/usr/bin/env bash

export DYLD_LIBRARY_PATH=$LIBRARY_PATH
substreams-sink-kv inject ${1:-eos.firehose.eosnation.io:9001} "badger3://$(pwd)/badger_data.db" substreams.yaml
