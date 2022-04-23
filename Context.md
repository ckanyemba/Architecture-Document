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

![Alt text](https://www.github.com/ckanyemba/Architecture-Document/Diagrams/C4_Model/C4_Context.svg "a title")

### 2.2. Product Stakeholders

Actors are stakeholders that interact with product directly.

| Stakeholder Name         | Actor (Yes/No) | Concern  |
|--------------------------|----------------|----------|
| Ford Company             | No             | Get the SDL system with enough quality and functionality that fulfill their goals |
| PM / Architect / Analyst | No             | Use Customer Requirements Specification |
| Developers               | Yes            | Construct and deploy the system from specifications |
| Testers                  | No             | Test the system to ensure that it is suitable for use |

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
