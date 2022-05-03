import libp2p_rust_python_bindings as libp2p


def main():
    keypair = libp2p.identity.Keypair.generate_ed25519()
    print(keypair)
    public_key = keypair.public()
    print(public_key)
    peer_id = libp2p.PeerId(public_key=public_key)
    print(peer_id)


if __name__ == "__main__":
    main()
