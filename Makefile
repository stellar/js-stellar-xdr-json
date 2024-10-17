build: clean
	wasm-pack build --release --target web
	sed -i'.bak' -e 's#"stellar-xdr-json"#"@stellar/stellar-xdr-json"#' pkg/package.json
	ls -lah pkg/*.wasm

clean:
	rm -fr pkg

test:
	cargo test --test tests
	wasm-pack test --headless --firefox
