POST http://localhost:8080/tables/3/items
{
    "items_names": ["Pizza", "Pasta", "Salad", "Ramen", "Soup"]
}
HTTP 201
[Asserts]
jsonpath "$.status" == "success"

GET http://localhost:8080/tables/3/items
HTTP 200
[Asserts]
jsonpath "$" count >= 5

GET http://localhost:8080/tables/3
HTTP 200
[Asserts]
jsonpath "$.table_number" == 3
jsonpath "$.items" count >= 5
