# xdiff

## command

```shell
cargo run -- run -p rust -c fixtures/test.yml -e a=100
```

## examples

```yml
---
todo:
  req1:
    url: https://jsonplaceholder.typicode.com/todos/1
    params:
      a: 100
  req2:
    url: https://jsonplaceholder.typicode.com/todos/2
    params:
      c: 200
  res:
    skip_headers:
      - report-to
rust:
  req1:
    method: GET
    url: https://www.rust-lang.org/
    headers:
        user-agent: Aloha
    params:
      hello: world
  req2:
    method: GET
    url: https://www.rust-lang.org/
    params: {}
  res:
    skip_headers:
      - set-cookie
      - date
      - via
      - x-amz-cf-id
```