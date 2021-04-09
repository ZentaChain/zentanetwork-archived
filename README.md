# :zap: Zentanetwork :zap:

Zentanetwork is a forkless Proof of Stake(PoS) consensus for communication and data storage. 
The construction of a forkless network enables Zentalk and Zentavault to make the perfect provision for Zentachain ecosystem.

## Status
Active under development.

## Build the $CHAIN

````
$ git clone https://github.com/ZentaChain/Zentanetwork
````

Fast Build check

````
$ cargo test --all
````

Getting Started with Zentanetwork

````
$ cargo build --release
````

## Test-networks

### Local-network

````
$ ./target/release/zentachain --local
````
### Multi-local-network
Alice

````
$ ./target/release/zentachain --chain local --alice /tmp/alice
````
Bob
````
./target/release/zentachain chain local --bob /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/BOOTNODEID'
````
## Dev-network
Run single-node in development network

````
$ ./target/release/zentachain --dev
````

## Zajin-network (Alpha)
Run single-node in Zajin-network
````
$ ./target/release/zentachain --zajin
 ````
 
## Zikaron-network (Beta)
Run single-node in Zikaron-network
````
$ ./target/release/zentachain --zikaron
````

## Zentachain Main-network (Aleph-Network)

````
$ ./target/release/zentachain --mainnet
````

### Purge DB
Purging chain-database

````
$ ./target/release/zentachain purge-chain --"chainID"
````
### Help
For a breakdown of substrate command-line options
````
$ ./target/release/zentachain --help
````

## Advanced Introduction 
For deeper insights into the [Zentanetwork Docs](https://docs.zentachain.io/zentanetwork)

## License

[GNU Affero General Public License v3.0](https://github.com/ZentaChain/Zentanetwork/blob/master/LICENSE)
