# kuack-checker

A simple, platform-agnostic HTTP endpoint performance checker written in Rust.

It compiles to both native binaries (Linux) and WebAssembly (WASM) for browser usage.

## Usage

### Download And Run (Linux)

Download and run the latest binary from the [Releases page](https://github.com/kuack-io/checker/releases).

```bash
curl -L -O https://github.com/kuack-io/checker/releases/latest/download/kuack-checker-linux-x86_64.tar.gz
tar -xzf kuack-checker-linux-x86_64.tar.gz
TARGET_URL=https://kuack.io ./kuack-checker
```

### Download And Run (Browser)

Download and run the latest WASM bundle from the [Releases page](https://github.com/kuack-io/checker/releases). Requires python3.

```bash
curl -L -O https://github.com/kuack-io/checker/releases/latest/download/kuack-checker-wasm32-web.tar.gz
tar -xzf kuack-checker-wasm32-web.tar.gz
python3 -m http.server 8087
```

Then open <http://localhost:8087/test-browser.html>

### Docker (Linux)

```bash
docker run --rm -e TARGET_URL=https://kuack.io ghcr.io/kuack-io/checker:latest
```

### Docker (Browser)

The WASM image contains the static assets. You can extract them to run locally. Requires python3.

```bash
id=$(docker create ghcr.io/kuack-io/checker:wasm-latest)
docker cp $id:/pkg .
docker cp $id:/test-browser.html .
docker rm -v $id
python3 -m http.server 8087
```

Then open <http://localhost:8087/test-browser.html>

### From Sources (Linux)

```bash
make run-linux
```

### From Sources (Browser)

Requires python

```bash
make run-browser
```

This will start a local server and open the test page in your default browser.
