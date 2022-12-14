## **FlgihtPath Microservice**

## Story

There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people, and different carrier/agency groups it can be hard to track where a person might be. In order to determine the flight path of a person, we must sort through all of their flight records.

## Goal

To create a microservice API that can help us understand and track how a particular person flight path. The API should accept a request that includes a list of flights, which are defined by a source and destination airport code. These flights may not be listed in order and will need to be sorted to find the total flight paths starting and ending airports.

## Examples

[['SFO', 'EWR']] => ['SFO', 'EWR']

[['ATL', 'EWR'], ['SFO', 'ATL']] => ['SFO', 'EWR']

[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] => ['SFO', 'EWR']

## How to install && run the microservice

### From source
If you have already installed rust environment, you can just run it from source code.
```terminal
git clone git@github.com:chainbelief/flightpath.git
cd flightpath
cargo run
```
### Using docker
You can build docker image using Dockerfile and run image on server platform
Build docker image

## How to test

### Using unit tests

```terminal
    cargo test
```

### Using http request

Please install RestClient vs code extension.
And run the http request in test.http file

# API Specs

- **URL**

  http://localhost:8000/flightpath/

- **Method:**

  `POST`

- **URL Params**

- **Data Params**

  ```javascript
    {
        "tickets": [
            { "src": "src1", "dst": "dst1", },
            { "src": "src2", "dst": "dst2", },
            { "src": "src3", "dst": "dst3", },
            ...
            ]
    }
  ```

- **Success Response:**

  - **Code:** 200 <br />
    **Content:** `{ "src" : "final_start", "dst": "final_end" }`

- **Error Response:**

  - **Code:** 404 Bad Request <br />
    **Content:** `{ "error" : "Invalid Path" }`

- **Sample Call:**

  ```javascript
    POST http://localhost:8000/flightpath HTTP/1.1
    Content-Type: application/Json;

    {
        "tickets": [
            {
                "src":"ABC",
                "dst":"DEF"
            },
            {
                "src":"DEF",
                "dst":"GCD"
            },
            {
                "src":"GCD",
                "dst":"LCD"
            }
        ]
    }

    Response

    {
        "src": "ABC",
        "dst": "LCD"
    }
  ```

- **Notes:**

## Estimation

This project took an hour to write including documentation and tests.
