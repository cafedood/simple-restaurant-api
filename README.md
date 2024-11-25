# Simple Restaurant API 

## Assumptions
1. Allow adding items with the same name to a table, as in real life customers can order the same item multiple times. 
2. When ordered mulitple items with the same name in a table, 

   a. `Get the item in a table` will return all items which matched the name
   b. `Delete the item in a table` will remove all items which matches the name

## Run the service
```sh
$ cargo clean && cargo build && cargo run 
```
Logs: 
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
Running `target/debug/simple-restaurant-api`
```

## API Tests 

### Add new items
```sh
$ curl -X POST 'localhost:8080/tables/1/orders' \
       -H 'Content-Type: application/json' \
       -d '["Pizza", "Pasta", "Salad"]'  
```
Response:
```json
{
  "items": [
    {
      "name": "Pizza",
      "cooking_time": 14
    },
    {
      "name": "Pasta",
      "cooking_time": 12
    },
    {
      "name": "Salad",
      "cooking_time": 10
    }
  ]
}
```

### Get all items for a table
```sh
$ curl 'localhost:8080/tables/1/orders'
```
Response: 
```json
{
  "items": [
    {
      "name": "Pizza",
      "cooking_time": 14
    },
    {
      "name": "Pasta",
      "cooking_time": 12
    },
    {
      "name": "Salad",
      "cooking_time": 10
    }
  ]
}
```

### Delete the item in a table 
```sh
$ curl -X DELETE 'localhost:8080/tables/1/orders/Pizza'
```
Response: 
```json
{
  "items": [
    {
      "name": "Pasta",
      "cooking_time": 12
    },
    {
      "name": "Salad",
      "cooking_time": 10
    }
  ]
}
```

### Get the item in a table
```sh
$ curl 'localhost:8080/tables/1/orders/Pasta'
```
Response:
```json
{
  "name": "Pasta",
  "cooking_time": 12
}
```
```sh
$ curl 'localhost:8080/tables/1/orders/Pizza'
```
Response:
```json
Item not found
```


