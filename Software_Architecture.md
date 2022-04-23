# TurtlePay® Service Delivery Architecture

## Objective

This document provides the working standard for the infrastructure design and service delivery model of the TurtlePay® project. While this document does serve as an outline and model for how the services will be designed and ultimately provided, it is subject to change without notice and actual service delivery models may differ from this document.

Our goal; however, is to maintain concurrency between the service platform and this document for all to enjoy.

![C4_Container](https://user-images.githubusercontent.com/95967301/164893525-5d198b28-0145-4b43-ab44-4d622c900cde.svg)


## Infrastructure

Connecting to the TurtleCoin® network requires utilizing the `TurtleCoind` binary to provide core services. As this is our gateway to the network, the stability, reliability, and reachability of the node services is one of the most important things for TurtlePay® services.

To ensure [99.999%](https://en.wikipedia.org/wiki/High_availability#Percentage_calculation) or higher uptime for the services the infrastructure must be designed with high-availability in mind. To accomplish this, multiple redundant layers including [inter-process communication](https://en.wikipedia.org/wiki/Inter-process_communication) between those layers is required.

### TurtleCoin® Node Deployment

Nodes will be deployed in localized clusters. The clusters will consist of not less than 3 nodes at any given time. For the highest availability on the current TurtleCoin® software, Microsoft Windows hosts will be used.

Each cluster will consist of the nodes and a high-availability load balancer that verifies the state of the node as suitable. Each load balancer is charged with making sure that the cluster is always available.

***Note:*** TurtlePay® and the underlying wallet(s) used for the service will, at no point, directly communicate with **any** nodes.

### Blockchain Database Storage System (BDSS)

To store the blockchain, multiple candidate database systems are being evaluated.

Core factors to consider in database system selection include [ACID](https://en.wikipedia.org/wiki/ACID_(computer_science)) properties:

* Atomicity
* Consistency
* Isolation
* Durability

Candidate systems include, but are not limited to:

* [MySQL](https://dev.mysql.com/)
* [MariaDB](https://mariadb.org/)
* [PostgreSQL](https://www.postgresql.org/)
* [SQLite](https://www.sqlite.org/index.html)
* [Microsoft SQL Server](https://www.microsoft.com/en-us/sql-server/default.aspx)

***Note:*** The selection of a standard cloud provider for this portion of the service is acceptable as the information contained within their data store is **public** knowledge that exists already on the blockchain.

The highly scalable database will be used in lieu of working directly with the TurtleCoin® software for a number of reasons, including:

* Provides a [DMZ](https://en.wikipedia.org/wiki/DMZ_(computing))-style buffer between the TurtleCoin® network and the TurtlePay® services
* Provides an [abstraction layer](https://en.wikipedia.org/wiki/Abstraction_layer) that does not rely on the underlying node software
* Provides faster random blockchain access than the node software permits at a much larger scale

### Standard Services Database

TurtlePay® requires the same level of database control and ACID properties as the blockchain storage for managing and maintaining information such as the following:

* Developer Account Services
* Application Endpoint Information
* Transaction Information Through the Service
* Audit Trails & Logs as Required
* Debugging, Troubleshooting, & Support Information

## Core Services

### [Blockchain Data Collection Agent (BDCA)](https://github.com/TurtlePay/blockchain-data-collection-agent)

To mitigate any issues with the node software, data (blocks & transaction mempool) will be collected from the clustered regions at standard intervals.

The BDCA(s) will poll the clusters for changes in the blockchain including re-organization events, new blocks, and transaction memory pool changes. Each instance of the BDCA will exist as close to the clusters as possible to minimize latency in the polling of the node.

This information, once collected, will be pushed into the BDSS for further consumption by TurtlePay® services.

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

![TurtlePay® v1 Process Flow](https://github.com/Architecture-Document/Diagrams/C4_Model/C4_Context.svg)

###### (c) 2018-2019 T![C4_Component](https://user-images.githubusercontent.com/95967301/164893510-a39b1180-07df-437c-8e0d-8bbef4e65607.svg)
urtlePay® Development Team
