# Spin Outbound PostgreSQL example

This example shows how to access a PostgreSQL database from Spin component.

## Spin up

From example root:

```
createdb spin_dev
psql -d spin_dev -f db/testdata.sql
RUST_LOG=spin=trace spin build --up
```

Curl the read route:

```
$ curl -i localhost:3000/read
HTTP/1.1 200 OK
transfer-encoding: chunked
date: Wed, 06 Nov 2024 20:17:03 GMT

Found 2 article(s) as follows:
article: Article {
    id: 1,
    title: "My Life as a Goat",
    content: "I went to Nepal to live as a goat, and it was much better than being a butler.",
    authorname: "E. Blackadder",
    published: Date(
        2024-11-05,
    ),
    coauthor: None,
}
article: Article {
    id: 2,
    title: "Magnificent Octopus",
    content: "Once upon a time there was a lovely little sausage.",
    authorname: "S. Baldrick",
    published: Date(
        2024-11-06,
    ),
    coauthor: None,
}

(Column info: id:DbDataType::Int32, title:DbDataType::Str, content:DbDataType::Str, authorname:DbDataType::Str, published:DbDataType::Date, coauthor:DbDataType::Str)
```

Curl the write route:

```
$ curl -i localhost:3000/write
HTTP/1.1 200 OK
content-length: 9
date: Sun, 25 Sep 2022 15:46:22 GMT

Count: 3
```
