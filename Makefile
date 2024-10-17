build: clean
	wasm-pack build --release --target web
	ls -lah pkg/*.wasm

clean:
	rm -fr pkg

test:
	cargo test --test tests
	wasm-pack test --headless --firefox
