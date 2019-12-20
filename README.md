## Using `Rust` to Access an `API` for JSON data

1. This is an introductory program for accessing json data using an api through Rust.

2. We are using mainly [serde](https://crates.io/crates/serde), [futures](https://crates.io/crates/futures) and [actix-web](https://crates.io/crates/actix-web) crates in this exercise.

3. Additional Tools:

    - We can use this [online](https://rusty-json.herokuapp.com/) help to convert JSON into `Rust` struct.
    - For continous development and viewing, we can use a very handy crate called [cargo-watch](https://crates.io/crates/cargo-watch). `Cargo Watch` watches over project's changes and runs `Cargo` commands when they occur. 

4. Compilation and execution can be done using: 
    - cargo watch -x 'build'
    - cargo watch -x 'run'

5. Since we are starting program on localhost, we can access it in browser at this url:
    - Get all json data: `http://127.0.0.1:4000/`
    - Get pollutant_avg: `http://127.0.0.1:4000/average`
    - Get pollutant_max: `http://127.0.0.1:4000/maximum`
    - Get pollutant_min: `http://127.0.0.1:4000/minimum`

6. Sample Output of program in browser looks as follows:
```json
-------------------------omitted json data-------------------
{
"id": "6",
"country": "India",
"state": "Andhra_Pradesh",
"city": "Amaravati",
"station": "Secretariat, Amaravati - APPCB",
"last_update": "18-12-2019 06:00:00",
"pollutant_id": "CO",
"pollutant_min": "30",
"pollutant_max": "68",
"pollutant_avg": "46",
"pollutant_unit": "NA"
},
{
"id": "7",
"country": "India",
"state": "Andhra_Pradesh",
"city": "Amaravati",
"station": "Secretariat, Amaravati - APPCB",
"last_update": "18-12-2019 06:00:00",
"pollutant_id": "OZONE",
"pollutant_min": "30",
"pollutant_max": "72",
"pollutant_avg": "44",
"pollutant_unit": "NA"
},
-------------------------omitted json data-------------------
```
