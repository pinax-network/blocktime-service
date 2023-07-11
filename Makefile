.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: wasm
wasm:
	cargo build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/blocktime.wasm .

.PHONY: clearDB
clearDB:
	rm -rf badger_data.db/

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 kv_out -s 2 -t +2 