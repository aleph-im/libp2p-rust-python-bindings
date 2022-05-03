import libp2p_rust_python_bindings as libp2p
import asyncio


async def main():
    keypair = libp2p.identity.Keypair.generate_ed25519()
    public_key = keypair.public()
    peer_id = libp2p.PeerId(public_key=public_key)

    transport = await libp2p.development_transport(keypair)
    print(transport)


if __name__ == "__main__":
    asyncio.run(main())
