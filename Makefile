build:
	wasm-pack build
	sed -i'.bak' -e 's#"stellar-xdr-json"#"@stellar/stellar-xdr-json"#' pkg/package.json

test:
	wasm-pack test --headless --firefox
