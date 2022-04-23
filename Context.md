# Context Diagram for Human App
## 2.  Case Background

### 2.1. System Context, Mission and Scope

hCaptcha is only the first application on the Protocol. The Protocol is designed to allow arbitrary task types to be designed, published, and purchased, allowing large groups of workers (“labor pools”) to cooperate with requesters of work without requiring high trust on both sides:

-   A Mobile Device might be connected via USB, Bluetooth or Wi-Fi to the HU;
-   The Application should be the SDL-enabled one.

The Mobile Device might be any of:
-   Smartphone devices
-   Tablet PCs

with operational system:
-   iOS
-   Android.

The SDL system allows Application to:
-   Use vehicle HMI: VR, TTS, buttons (physical and touch-screen), vehicle display, audio system. etc.
-   Retrieve Vehicle Data (seat belt position, transmission shift lever position, airbag status, etc.).

![alt text](https://github.com/ckanyemba/Architecture-Document/Diagram/C4_Model "C4_Context.svg")

### 2.2. Product Stakeholders

Actors are stakeholders that interact with product directly.

# HUMAN App


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


# Note for developers
Bootstrap framework is DEPRECATED. For newer components everyone should use material-ui ui-kit instead


### 2.3. Business Goals

Luxoft delivered to Ford a prototype of POSIX compliant Applink Core in March, 2013.
To support FORD goal of successful acceptance of Applink (new name is SmartDeviceLink) Core by open source community of GENIVI consortium further enhancements will be required. The purpose of the project is to develop component of SmartDeviceLink 4.x Core by adding new features required by Ford.

### 2.4. Significant Driving Requirements

The requirements are listed in the table below and ordered by descending of their significance from architectural solution point of view.

| \# | **Driving Requirement Description** |
|----|-------------------------------------|
| 1. | System has to be POSIX-compliant to be easily ported on all POSIX standardized OSs. |
| 2. | Transport for communication between Mobile Application and SDL system must be implemented and easily changed, replaced or added if required |
| 3. | APIs for communication between Mobile Application and SDL system described in appropriate documents have to be fully supported by the system. |
| 4. | There has to be relatively easy way to port existing HMI Modules (such as UI, VR, TTS, etc.) to work with SDL system. |
| 5. | APIs for communication between SDL system and HMI Modules have to be fully described in appropriate document and fully supported by SDL system. |
