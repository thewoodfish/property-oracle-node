
<img src="https://github.com/thewoodfish/property_oracle/blob/main/public/img/logo.png">

Property Oracle is a blockchain solution built on <a target="_blank" href="https://substrate.io">substrate</a> and <a target="_blank" href="https://kilt.io">kilt</a>,  which empowers you to create an indisputable, irrefutable proof of ownership of your various properties. e.g a plot of land.
With Property Oracle, nobody can take what is yours and leave you with nothing.

# Property Oracle Blockchain Node
This repo contains the code for a typical Property Oracle node participating in the network. The property oracle chain is very important because it records critical information about users of the networks, the properties being secured or claimed, the claims and right to claim of the properties and so on, serving as a source of truth and actions. We would examine the chain properly. A Property Oracle node is completely built on <a target="_blank" href="https://substrate.io">substrate</a> which is the best framework for building a blockchain.

## Examining the `oracle pallet`
Since this chain is build on <a target="_blank" href="https://substrate.io">substrate</a>, it comprises of <a target="_blank" href="https://substrate.io">pallets</a> which defines the business logic of the chain. We would be considering one very important property oracle pallet: the `oracle pallet`.

### Functions in the `oracle pallet`

