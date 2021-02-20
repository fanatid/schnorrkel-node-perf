bench: build
	export SEED=DWHFeX6QLfsy3pe6xoCH0ONdsnAgNjv2 && cd schnorrkel-bench && cargo bench && npm run bench

build: format
	cd schnorrkel-bench && cargo bench --no-run
	cd schnorrkel-napi && npm run build
	cd schnorrkel-neon && npm run build
	cd schnorrkel-wasm && make build

format:
	cd schnorrkel-bench && cargo-fmt && npm run format
	cd schnorrkel-napi && cargo-fmt
	cd schnorrkel-neon/native && cargo-fmt
	cd schnorrkel-wasm && cargo-fmt

clean:
	rm -rf \
		target node_modules \
		schnorrkel-napi/schnorrkel-napi.node \
		schnorrkel-neon/native/artifacts.json schnorrkel-neon/native/index.node \
		schnorrkel-wasm/pkg
