## sqlx mssql test
> Modified the [sqlx todo actix-web example](https://github.com/actix/examples/tree/master/sqlx_todo) to work with sql server.

To run:
1. `docker-compose up -d`
2. Give the scripts a few seconds to run to set things up :)
3. `cargo run`
4. Hit [http://127.0.0.1:8080/](http://127.0.0.1:8080/) in your browser.

## API
### Crete TODO
```bash
curl -v \
-H 'content-type: application/json' \
-H 'accept: application/json' \
-d '{"description":"Create a simple POST/INSERT example curl","done":true}' \
http://127.0.0.1:8080/todo
```

### Delete TODO
```bash
curl -v -X DELETE \
-H 'content-type: application/json' \
-H 'accept: application/json' \
http://127.0.0.1:8080/todo/1
```

### Update TODO
```bash
curl -v -X PUT \
-H 'content-type: application/json' \
-H 'accept: application/json' \
-d '{"description":"Create a simple PUT/UPDATE example curl","done":true}' \
http://127.0.0.1:8080/todo/1
```

### Get a TODO
```bash
curl -v \
-H 'accept: application/json' \
http://127.0.0.1:8080/todo/1
```

### Get all TODOs
```bash
curl -v \
-H 'accept: application/json' \
http://127.0.0.1:8080/todo
```
