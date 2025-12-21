.PHONY: build-linux build-browser build-all run-linux run-browser fmt fmt-fix lint test coverage deny audit check dependencies clean clear package-linux package-wasm

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/kuack-checker ./kuack-checker-linux-x64

build-browser:
	wasm-pack build --target web --release --out-dir pkg

build-all: build-linux build-browser

run-linux: build-linux
	TARGET_URL=https://kuack.io ./kuack-checker-linux-x64

run-browser: build-browser
	@echo "Stopping any existing server on port 8080..."
	@lsof -ti:8080 | xargs kill -9 2>/dev/null || true
	@sleep 0.5
	@echo "Starting HTTP server..."
	@python3 -m http.server 8080 > /dev/null 2>&1 &
	@sleep 1
	@echo "Server running at http://localhost:8080/test-browser.html"
	@command -v xdg-open > /dev/null 2>&1 && xdg-open http://localhost:8080/test-browser.html 2>/dev/null || open http://localhost:8080/test-browser.html 2>/dev/null || echo "Please open http://localhost:8080/test-browser.html in your browser"

fmt:
	cargo +nightly fmt --check

fmt-fix:
	cargo +nightly fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings

audit:
	cargo audit

deny:
	cargo deny check

test:
	cargo test --all-features

coverage:
	cargo tarpaulin --out Xml --fail-under 90 --exclude-files src/main.rs

check: fmt lint audit deny test coverage

dependencies:
	cargo update

package-linux:
	sudo docker buildx build --target runtime-linux -t kuack-checker:linux --load .

package-wasm:
	sudo docker buildx build --target runtime-wasm -t kuack-checker:wasm --load .

clean:
	cargo clean
	rm -f kuack-checker-linux-x64 kuack-checker-wasip2.wasm kuack-checker-browser.wasm
	rm -rf pkg

clear: clean
