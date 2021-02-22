# :zap: Zentanetwork :zap:

Zenta is a Substrate-based, forkless PoS blockchain for communication and data storage. 
The construction of a fork-free blockchain with Substrate enables Zentalk and Zentavault to make the perfect provision for Zentachain ecosystem. Zentameshnet is allow Zentalk user communication offline 2 offline.

### Build the $CHAIN

Fast Build check

````
cargo test --all
````

Getting Started with Zentanetwork

````
cargo build --release
````

## Test-networks

### Local-network

````
./target/release/zentachain --testnet-local
````
### Multi-local-network

````
./target/release/zentachain --testnet-local --alice -d /tmp/alice
````

````
./target/release/zentachain --testnet-local --bob -d /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/BOOTNODEID'
````
### Dev-network
Run single-node in development network

````
 ./target/release/zentachain --dev
````

### Zajin-network (Alpha)
Run single-node in Zajin-network
````
 ./target/release/zentachain --zajin
 ````
 
### Zikaron-network (Beta)

````
 ./target/release/zentachain --zikaron
````

### Zentachain Main-network (Aleph-Network)

````
./target/release/zentachain --mainnet
````

### Purge DB
Purging chain-database

````
./target/release/zentachain purge-chain --chain=ID
````
### Help
For a breakdown of substrate command-line options
````
./target/release/zentachain --help
````

### Advanced Introduction 
For deeper insights into the Zentanetwork
[DocHub](https://docs.zentachain.io)

### License

[GNU Affero General Public License v3.0](https://github.com/ZentaChain/Zentanetwork/blob/master/LICENSE)
