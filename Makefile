build:
	wasm-pack build --release --target web --out-dir pkg/web/
	sed -i'.bak' -e 's#"stellar-xdr-json"#"@stellar/stellar-xdr-json-web"#' pkg/web/package.json
	wasm-pack build --release --target deno --out-dir pkg/deno/
	ls -lah pkg/**/*.wasm

test:
	wasm-pack test --headless --firefox
