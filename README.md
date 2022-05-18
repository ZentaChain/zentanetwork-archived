# :zap: Zentanetwork :zap:

Zentanetwork is a forkless Proof of Stake (PoS) consensus for communication and data storage.
The construction of a forkless network enables Zentalk and Zentavault to make the perfect provision for Zentachain ecosystem.

### Getting Started Zentanetwork

```sh
# Clone Zentanetwork repository
git clone https://github.com/ZentaChain/Zentanetwork
```

```sh
# This command is a fast check
cargo test --all
```

```sh
# This command will firstly compile the code
cargo build --release
```

### Local-network

```bash
# Start the local-net
./target/release/zentachain --local
```

### Multi local-network

```bash
# Start Alice
./target/release/zentachain --chain local --alice /tmp/alice
````

```bash
# Start Bob with the boostnode id
./target/release/zentachain chain local --bob /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/BOOTNODEID'
```

### Development-network

```bash
# Run single-node development-net
./target/release/zentachain --dev
```

### Zajin-network (Alpha - POA)

````bash
# Run single-node Zajin-net
$ ./target/release/zentachain --chain zajin
 ````

### Zentachain Main-network (Zikaron - POS)

```bash
# Start the main-net
./target/release/zentachain --chain zikaron
```

### Zentachain Main-network (Alnitak - POS)

```bash
# Start the main-net of Alnitak
./target/release/zentachain --chain alnitak
```

### Purge Database of the Network

```bash
# Purging chain-database
./target/release/zentachain purge-chain --"chain-ID"
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

```bash
# Run node without re-compiling
./scripts/docker_run.sh ./target/release/zentachain --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/zentachain purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

### Help

```shell
# For a breakdown of the node command-line options
./target/release/zentachain --help
```

### Advanced Introduction 
For deeper insights into the [Zentanetwork](https://docs.zentachain.io/zentanetwork)

### Status
Under active development.

## License

[GNU Affero General Public License v3.0](https://github.com/ZentaChain/Zentanetwork/blob/master/LICENSE)

![license](https://img.shields.io/github/license/ZentaChain/Zentanetwork)
