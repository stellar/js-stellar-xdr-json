build: build-web build-deno

build-web: clean
	wasm-pack build --release --target web
	sed -i'.bak' -e 's#"stellar-xdr-json"#"@stellar/stellar-xdr-json-web"#' pkg/package.json
	ls -lah pkg/*.wasm

build-deno: clean
	wasm-pack build --release --target deno --out-dir pkg-deno
	ls -lah pkg/**/*.wasm

clean:
	rm -fr pkg pkg-deno

test:
	wasm-pack test --headless --firefox
