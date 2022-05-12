##  Development

# Human API Server

## Overview
This server is based on the spec defined [here](https://app.swaggerhub.com/apis/excerebrose/human-protocol/1.0.0#/)

## Requirements
Python 3.7.2+

## Usage
To run the server, please execute the following from the root directory:

```
./bin/run
```

Your Swagger definition will be here:

```
http://localhost:8080/swagger.json
```

To run the integration tests, use:
```
./bin/test
```


### 1. Implementation Technologies

- Javascript, Solidity, reactjs, rust, HTML, CSS  is selected as programming languages for Human Protocol architecture independent.


### 4.8.3. Development Environment and Standards
-   Minimum development and testing environment for Ubuntu:
    -   Debug Environment: Ubuntu 16.04 LTS x32/x64, Qt 5.3
    -   Compiler: GCC 5.3.1 (OS Ubuntu), Lua 5.2
    -   Build system: Cmake 3.10.2
-   Recommended development and testing environment for Ubuntu:
    -   Debug Environment: Ubuntu 18.04 LTS x32/x64, Qt 5.3
    -   Compiler: GCC 7.3.0 (OS Ubuntu), Lua 5.2
    -   Build system: Cmake 3.10.2
-   Development and testing environment for SDL Windows x64:
    -   Build system: Windows 7 x64, CMake
    -   Compiler: Microsoft Visual Studio Express Edition 2013 x64
-   Development and testing environment for SDL Qt for Windows x32:
    -   Build system: Windows 7 x32, Qt 5.5, CMake, QT Creator
    -   Compiler: Microsoft Visual Studio Express Edition 2010 x32
-   Requirements Management system: LuxProject (JIRA, Confluence)
-   Source Control System: GitHub
-   Issue Tracking System: LuxProject (JIRA)
-   Document Management System: LuxProject (JIRA, Confluence, SVN)
-   Coding style: [*SDL C++ Style*](https://github.com/smartdevicelink/sdl_core/wiki/SDL-Coding-Style-Guide)
