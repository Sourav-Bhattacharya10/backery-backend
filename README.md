# Backery Backend

The spelling of the name has been intentionlly kept wrong.

It is just practice project to get started with Sea ORM, Rocket and Rust.

It has 3 endpoints as of now:
* GET http://localhost:8000/health
* GET http://localhost:8000/bakeries
* POST http://localhost:8000/bakeries\
Request Body:
{
    "name": "Bakery1",
    "profit_margin": 1.0
}