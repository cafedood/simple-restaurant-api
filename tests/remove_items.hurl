POST http://localhost:8080/tables/1/items
{
    "items_names": [
        "Pizza"
    ]
}
HTTP 201
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.items_ids" count >= 1
[Captures]
item_id: jsonpath "$.items_ids[0]"

DELETE http://localhost:8080/tables/1/items/{{item_id}}
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed item id: {{item_id}}" 