# OpenAPI to HTTP
Converter that takes OpenAPI and generates HTTP files from it.

Main purpose of this converter is to generate files,
which then neovim plugin [rest.nvim](https://github.com/rest-nvim/rest.nvim) can use for curl requests.

# Usage
Following command should generate HTTP files in `requests` directory.
```
open-api-to-http --output ./requests --schema my-open-api-schema.json
```

OpenAPI schema - `my-open-api-schema.json`

```
{
  "paths": {
    "/customers": {
      "post": {
        "responses": {
          "201": {
            "description": "Created"
          }
        },
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "name": {
                    "type": "string"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

HTTP file - `customers.http`

```
# Body
#  - name?: String
#
POST /customers
host: {{HTTP_HOST}}
Content-Type: application/json
```
