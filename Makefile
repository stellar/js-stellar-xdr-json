build: clean
	wasm-pack build --release --target web
	deno --allow-read=pkg/ --allow-write=pkg/ scripts/post-build.ts
	ls -lah pkg/*.wasm pkg/*.js

clean:
	rm -fr pkg

test:
	cargo test --test tests
	wasm-pack test --headless --firefox
