POST http://localhost:8080/tables/1/items
{
    "items_names": ["Pizza", "Pasta", "Salad", "Ramen", "Soup"]
}
HTTP 201
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Added 5 items on table 1"
jsonpath "$.items_ids" count == 5
