.PHONY: all
all:
	make wasm
	make pack
	make graph
	make info

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
