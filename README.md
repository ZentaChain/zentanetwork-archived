  ![last-commit](https://img.shields.io/github/last-commit/Zentachain/Zentanetwork)
# :zap: Zentanetwork :zap:

Zentanetwork is a forkless Proof of Stake (PoS) consensus for communication and data storage. 
The construction of a forkless network enables Zentalk and Zentavault to make the perfect provision for Zentachain ecosystem.

## Status
Under active development.

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

## Zentachain Main-network (Zikaron-network)

````
$ ./target/release/zentachain --zikaron
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
### M1 Machine - Build Error

If you are using a M1 (System-on-a-Chip) in with machine please visit before building the node [Apple M1 Error Handling](https://docs.zentachain.io/zentanetwork#apple-m1-compile-error-handling) otherwise you will not be able to build and compile it.

## Advanced Introduction 
For deeper insights into the [Zentanetwork Docs](https://docs.zentachain.io/zentanetwork)

## License

[GNU Affero General Public License v3.0](https://github.com/ZentaChain/Zentanetwork/blob/master/LICENSE)
 ![license](https://img.shields.io/github/license/ZentaChain/Zentanetwork)
