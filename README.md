# :zap: Zentanetwork :zap:

Zenta is a Substrate-based, forkless PoS blockchain for communication and data storage. 
The construction of a fork-free blockchain with Substrate enables Zentalk and Zentavault to make the perfect provision for Zentachain ecosystem. Zentameshnet is the secret part to allow Zentalk user communication offline 2 offline. Staking with Zentanodes will uphold the Zentameshnet.

### Build the $CHAIN

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

### Dev-network
Run single-node in development network 
````
 ./target/release/zentachain --dev
````

### Zajin-network (Alpha)

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

````
./target/release/zentachain --help
````

### Advanced Introduction 

[DocHub](https://docs.zentachain.io)

### License

[GNU Affero General Public License v3.0](https://github.com/ZentaChain/Zentanetwork/blob/master/LICENSE)
