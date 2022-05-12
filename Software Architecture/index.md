# Human Protocol Architecture

## Objective
 
> * This document provides the working standard for the infrastructure design and service delivery model of the Human-Protocol project. While this document does serve as an outline and model for how the services provided, it is subject to change without notice and actual service delivery models may differ from this document.

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

> * As with any blockchain system, a wallet must be maintained. The core functionality of a wallet is as follows:
* Hold Public & Private Keys
* Scan blockchain transactions
  * Retrieve outputs meant for our keys
  * Track spendable outputs
  * Create new inputs from spendable outputs
* Generate & Sign new transactions

> * HUMAN Protocol allows Exchanges to publish arbitrary job types, but it also defines many standard job types that serve as building blocks for a multitude of tasks. This becomes interesting when we consider an area of active research and development today, factored cognition : decomposing complicated work into simple components

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

A sandbox mode for all API requests will be provided to all developers to aide in the development and testing of applications built on the Human Protocol® platform.

## Service Delivery Model

The following diagram has been created to document the design concept driving [Phase 2](https://github.com/TurtlePay/architecture/blob/master/Roadmap.md#phase-2).

![C4_Component](https://user-images.githubusercontent.com/95967301/164978032-1de41ece-2ed1-4a1f-867c-fa1bb9dfc58f.svg)

|---|---|
|[Resiliency]|Because the business logic is controlled by a smart contract, a DApp backend will be fully distributed and managed on a blockchain platform. Unlike an application deployed on a centralized server, a DApp will have no downtime and will continue to be available as long as the platform is still operating.|
|[Transparency]|The on-chain nature of a DApp allows everyone to inspect the code and be more sure about its function. Any interaction with the DApp will be stored forever in the blockchain.|
|[Censorship resistance]|As long as a user has access to an Ethereum node (running one if necessary), the user will always be able to interact with a DApp without interference from any centralized control. No service provider, or even the owner of the smart contract, can alter the code once it is deployed on the network.|

> * In the Ethereum ecosystem as it stands today, there are very few truly decentralized apps—most still rely on centralized services and servers for some part of their operation. In the future, we expect that it will be possible for every part of any DApp to be operated in a fully decentralized way.

###### (c)
