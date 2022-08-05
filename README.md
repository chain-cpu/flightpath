## API Specs

## **Title**

<_Additional information about your API call. Try to use verbs that match both request type (fetching vs modifying) and plurality (one vs multiple)._>

- **URL**

  http://localhost:8000/flightpath/

- **Method:**

  `POST`

- **URL Params**

- **Data Params**
  `{ "tickets": [ { "src": "src1", "dst": "dst1", }, { "src": "src2", "dst": "dst2", }, { "src": "src3", "dst": "dst3", }, ... ] }`

- **Success Response:**

  - **Code:** 200 <br />
    **Content:** `{ "src" : "final_start", "dst": "final_end" }`

- **Error Response:**

  - **Code:** 404 Bad Request <br />
    **Content:** `{ "error" : "Invalid Path" }`

- **Sample Call:**

POST http://localhost:8000/flightpath HTTP/1.1
Content-Type: application/json

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

- **Notes:**

## Estimation

This project took an hour to write including documentation and tests.

## Story

There are over 100,000 flights a day, with millions of people and cargo being transferred around the world. With so many people, and different carrier/agency groups it can be hard to track where a person might be. In order to determine the flight path of a person, we must sort through all of their flight records.
