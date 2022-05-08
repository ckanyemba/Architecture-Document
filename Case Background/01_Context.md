# Context Diagram for Human Protocol
## 1.  Case Background

### 1.1. System Context, Mission and Scope

hCaptcha is only the first application on the Protocol. The Protocol is designed to allow arbitrary task types to be designed, published, and purchased, allowing large groups of workers (“labor pools”) to cooperate with requesters of work without requiring high trust on both sides:

- Requesters of work launch new bounties onto the blockchain that specify a job: the question to ask and the set of tasks to ask it about.
- Exchanges pick up jobs, manage bidding on job types, and serve tasks to agents doing the work.
- Recording Oracles collect potential answers and provide a rolling evaluation of answer quality.
- Reputation Oracles make a final evaluation of answer quality and reputation score per job, and finally pay out bounties

![C4_Context](https://user-images.githubusercontent.com/95967301/164949698-7e7754e8-f9c2-40d3-b46d-ebf54032c5f9.svg)


### 2.1. Product Stakeholders
 - i) Requesters of work: these are humans or machines who pay a fee in HMT to launch jobs on the blockchain. 
 
 - ii) Workers, typically humans, who are compensated in HMT for completing those jobs

## Description

The HUMAN App is the frontend interface which allows earners to connect to HUMAN Protocol and solve labeling tasks for $HMT rewards. 
It is written in the React.js. Please visit [humanprotocol.org](https://humanprotocol.org) for more information

## Note:
The application is in active development state and can have breaking changes.

### Prerequisites:

* `REACT_APP_API_URL`: Human APP API
* `REACT_APP_HCAPTCHA_SITE_KEY`: Site Key for HCaptcha widget
* `REACT_APP_CIVIC_APP_ID`: Civic Integration APP ID


### Running locally:
$ `cp .env.example .env`

$ `yarn`

$ `yarn start`

### Running in the production:
$ `yarn`

$ `yarn build`

$ `REACT_APP_API_URL=${API_URL} PORT=${PORT} yarn start-prod`


