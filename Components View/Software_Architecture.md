# Human Protocol Architecture

## Objective
 
 - This document provides the working standard for the infrastructure design and service delivery model of the TurtlePay® project. While this document does serve as an outline and model for how the services will be designed and ultimately provided, it is subject to change without notice and actual service delivery models may differ from this document.

Our goal; however, is to maintain concurrency between the service platform and this document for all to enjoy.


![C4_Container](https://user-images.githubusercontent.com/95967301/164977167-e33e6200-e163-4cd2-99b0-1a48ab3564f3.svg)


## Infrastructure

HUMAN Tokens (“HMTs”) serve as the medium of exchange in the HUMAN Protocol. They are EIP20-compatible tokens, and the complete system forms a decentralized platform with an open protocol. Each component receives a fee for its role, and interactions are coordinated via smart bounties on the Ethereum blockchain


### Blockchain Database Storage System (BDSS)

To store the blockchain, multiple candidate database systems are being evaluated.

* Atomicity
* Consistency
* Isolation
* Durability



### [TurtleCoin® Wallet Service (TWS)](https://github.com/TurtlePay/turtlepay-wallet)

As with any blockchain system, a wallet must be maintained. The core functionality of a wallet is as follows:

* Hold Public & Private Keys
* Scan blockchain transactions
  * Retrieve outputs meant for our keys
  * Track spendable outputs
  * Create new inputs from spendable outputs
* Generate & Sign new transactions

Our wallet(s), instead of relying on a service such as `turtle-service` will live as native services that interact not with the node but with the data collected by the BDCAs. The wallet(s) will scan transactions and the transaction memory pool directly from the database.

Benefits of working with the data in the database instead of a traditional wallet include, but are not limited to:

* Provides a [DMZ](https://en.wikipedia.org/wiki/DMZ_(computing))-style buffer between the TurtleCoin® network and the TurtlePay® services
* Provides an [abstraction layer](https://en.wikipedia.org/wiki/Abstraction_layer) that does not rely on the underlying wallet software
* Fine grain atomic control of wallet operations
* Easy to scale and spin up on demand
  * Permits One Time Use (OTU) wallets

***Note:*** Development of a [Node.js](https://nodejs.org/) native wallet is underway to support this effort.

### [Blockchain Relay Agent (BRA)](https://github.com/TurtlePay/blockchain-relay-agent)

Each BRA is tasked with ensuring that new transactions from the TurtlePay® platform are properly relayed to the TurtleCoin® network for processing. Each transaction meant for network consumption is queued by the TurtlePay® platform before being broadcast to the TurtleCoin® network. The BRA is charged with verifying that the transaction has been accepted by a node and providing the state of such back to TurtlePay®.

***Note:*** Multiple outgoing transfers from the TurtlePay® platform may be combined into a single network transaction to make the most efficient use of block space.

## TurtlePay® Services

### [Public Website](https://turtlepay.io)

The front-end website provides general functionality similar to other payment gateway platforms such as, but not limited to:

* Automated Account Creation
* Account Maintenance
* Application Maintenance
* Security Services

### Developer Tools

#### [Application Programming Interface (API)](https://docs.turtlepay.io)

API services as used by developers will be provided via standard HTTP RESTful calls to a common gateway interface (i.e. https://api.turtlepay.io).

The API design will be outlined further in another document within this repository at a later date.

#### Standard Libraries

The project will provide a collection of standard utilities to interact with the TurtlePay® API. Different libraries will be made available upon request and within reason.

#### Example Applications

The project will provide a number of example applications including how to integrate TurtlePay® services into other applications or e-commerce platforms upon request and within reason.

#### Sandbox Mode

A sandbox mode for all API requests will be provided to all developers to aide in the development and testing of applications built on the TurtlePay® platform.

## Service Delivery Model

The following diagram has been created to document the design concept driving [Phase 2](https://github.com/TurtlePay/architecture/blob/master/Roadmap.md#phase-2).

![C4_Component](https://user-images.githubusercontent.com/95967301/164978032-1de41ece-2ed1-4a1f-867c-fa1bb9dfc58f.svg)



###### (c)
