# libp2p-rust Python bindings

Python bindings to the Rust implementation of libp2p.

## Dependencies

We use PyO3 for the bindings, and (obviously) rust-libp2p.

## Set up developer environment

You will need to install [maturin](https://github.com/PyO3/maturin) to set up the development environment.

```bash
python3 -m virtualenv venv
source venv/bin/activate
pip install maturin

# Installs the bindings 
maturin develop
```

Then, from Python:

```python
import libp2p_rust_python_bindings as libp2p
# Have fun!

keypair = libp2p.identity.KeyPair.generate_ed25519()
```

## Why bother with bindings when py-libp2p exists?

The [official Python implementation of libp2p](https://github.com/libp2p/py-libp2p) is unmaintained.
After discussing with libp2p maintainers, it appears that there is no plan to improve it in the near future.
In its current state, the library is pretty much unusable, so we decided to create bindings to the Rust
implementation, which is considered a first-class citizen by the libp2p maintainers.
