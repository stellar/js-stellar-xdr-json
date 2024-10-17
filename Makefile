build: clean
	wasm-pack build --release --target web
	node scripts/post-build.mjs
	ls -lah pkg/*.wasm pkg/*.js

clean:
	rm -fr pkg

test:
	cargo test --test tests
	wasm-pack test --headless --firefox
