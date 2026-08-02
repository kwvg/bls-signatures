[![GitHub License](https://img.shields.io/github/license/dashpay/bls-signatures)](https://github.com/dashpay/bls-signatures/blob/main/LICENSE)
[![Library status](https://img.shields.io/github/actions/workflow/status/dashpay/bls-signatures/build-test.yaml?branch=main&style=flat&logo=github&logoColor=white&label=library)](https://github.com/dashpay/bls-signatures/actions/workflows/build-test.yaml?query=branch%3Amain)
[![Binds status](https://img.shields.io/github/actions/workflow/status/dashpay/bls-signatures/build-binds.yml?branch=main&style=flat&logo=github&logoColor=white&label=binds)](https://github.com/dashpay/bls-signatures/actions/workflows/build-binds.yml?query=branch%3Amain)

> [!WARNING]
>
> It is heavily advised **against** using this library for new consensus implementations and to use established
> spec-conformant libraries like [supranational/blst](https://github.com/supranational/blst) as this library codifies
> primitives predating the final IETF spec and includes a non-standard (now legacy) scheme.
>
> **This library has not undergone a formal security review.**

`bls-signatures` is a cross-platform library implementing BLS12-381 primitives for Dash built on
the [`relic`](https://github.com/relic-toolkit/relic) toolkit with bindings available in
[Python](./binds/python), [Rust](./rust-bindings/), [Go](./go-bindings/) and [Javascript](./js-bindings).

## Dependencies

* A C++17 capable compiler (GCC 9, Clang 7 or higher)
* CMake 3.18 or higher (or Autoconf 2.71 or higher; with libtool and automake)
* [`libgmp`](https://gmplib.org/) (for fast arithmetic, **optional**)

Additionally, the following dependencies are supplied by the codebase

* [`catch2`](https://github.com/catchorg/Catch2) (for tests)
* [`mimalloc`](https://github.com/microsoft/mimalloc) (for secure memory operations)
* [`relic`](https://github.com/relic-toolkit/relic) (for cryptographic operations)

## Build library

```sh
# Create scratchpad directory
mkdir build && cd build

# Configure build files
cmake ..

# Build library with 4 threads
cmake --build . -- -j 4

# Run tests
./src/runtest

# Run benchmarks
./src/runbench
```

## Build Python binds

Our Python binds target Python 3.10 or higher; they depend on

* [`pybind11`](https://github.com/pybind/pybind11) (bridging C++ and Python)
* [`ruff`](https://github.com/astral-sh/ruff) (linting, part of optional `[.dev]` dependency group)

> [!NOTE]
> We recommend using programs like [`uv`](https://github.com/astral-sh/uv) to manage your virtualenv (`venv`) to prevent
> cross-contamination with Python-based native packages or other Python projects.

```sh
# Create a new venv named dashbls
uv venv dashbls

# Enter venv
source dashbls/bin/activate

# Install developer dependencies
uv pip install -e ".[dev]"

# Build binds
uv build

# Run linter and formatter
uv run ruff check
uv run ruff format --check

# Run unit tests
uv run python binds/python/test.py

# Run benchmarks
uv run python binds/python/benchmark.py
```

## License

```
Copyright (c) 2018-present, Chia Network, Inc.
Copyright (c) 2021-present, The Dash Core developers.
```
