
<img src="https://github.com/thewoodfish/property_oracle/blob/main/public/img/logo.png">

Property Oracle is a blockchain solution built on <a target="_blank" href="https://substrate.io">substrate</a> and <a target="_blank" href="https://kilt.io">kilt</a>,  which empowers you to create an indisputable, irrefutable proof of ownership of your various properties. e.g a plot of land.
With Property Oracle, nobody can take what is yours and leave you with nothing.

# Property Oracle Blockchain Node
This repo contains the code for a typical Property Oracle node participating in the network. The property oracle chain is very important because it records critical information about users of the networks, the properties being secured or claimed, the claims and right to claim of the properties and so on, serving as a source of truth and actions. We would examine the chain properly. A Property Oracle node is completely built on <a target="_blank" href="https://substrate.io">substrate</a> which is the best framework for building a blockchain.

## Examining the `oracle pallet`
Since this chain is build on <a target="_blank" href="https://substrate.io">substrate</a>, it comprises of <a target="_blank" href="https://substrate.io">pallets</a> which defines the business logic of the chain. We would be considering one very important property oracle pallet: the `oracle pallet`.

### Functions in the `oracle pallet`
- <b>`record_user:`</b>
   ```
   pub fn record_user(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult { ... }
   ``` 
This function accepts a `cid` as its input. The `cid` is generated when the `KILT did` document of a user is uploaded to `IPFS`. This `did` document is generated when a user creates an account with Property Oracle because its most certain that the user would be interacting with the <a target="_blank" href="https://kilt.io">`kilt`</a> blockchain in the background. What this function does is to store a mapping of the users `substrate account address` to the users `kilt` `did document` which contains all necessary info of the user.<br>
Once this state change is complete, the `NewUserRecorded` event is generated and visible on the blockchains UI explorer.

- <b>`record_ptype:`</b>
   ```
   pub fn record_ptype(
	origin: OriginFor<T>,
	hash: H256,
	name: Vec<u8>,
	cid: Vec<u8>,
	props: Vec<u8>,
   ) -> DispatchResult { ... } 
   ``` 
This function accepts the `id(hash)` of a property, the name or title of the property document to be filled, the `cid` of the uploaded `KILT chain` `CType` for the property document fields and the required attributes or fields to be filled in a property document.
This function creates a new property document and records it onchain. It also takes note of the registrar of the document. The registrar is important because his signature is the single principal entity to attest a property credential and confer the right meaning of ownership. <br>
Once this state change is complete, the `NewPropertyTypeRecorded` event is generated and visible on the blockchains UI explorer. 

- <b>`record_ptype:`</b>
   ```
   pub fn record_credential(origin: OriginFor<T>, hash: H256, cid: Vec<u8>) -> DispatchResult { ... }
   ``` 
This function accepts the `id(hash)` of a property and the `cid` of the `KILT` credential or claim created of a particular property. This function records the unverified claim to a property and the user or `address` of the claimer, waiting to be verified or approved.
Once this state change is complete, the `NewPropertyCredentialCreated` event is generated and visible on the blockchains UI explorer.


- <b>`transfer_property:`</b>
   ```
   pub fn transfer_property(
	origin: OriginFor<T>,
	recipient: T::AccountId,
	property_id: H256,
	cid: Vec<u8>,
   ) -> DispatchResult { ... } 
   ``` 
This functions accepts the `substrate address` of the recipient, the `id` of the intended property and the `cid` of the `KILT` credential or claim created of a particular property.
