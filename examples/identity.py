import libp2p_rust_python_bindings as libp2p


def main():
    keypair = libp2p.identity.PyKeypair.generate_ed25519()
    public_key = keypair.public()
    print(public_key)


if __name__ == "__main__":
    main()
