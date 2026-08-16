# Protocol

The kv-store speaks a line-based, text protocol. Each command is one line, terminated by a newline; each response is one line, also newline-terminated.

## Grammar

```
command   := "SET" WS key WS value
           | "GET" WS key
           | "DEL" WS key

key       := WORD | QUOTED_STRING
value     := WORD | QUOTED_STRING

WORD          := one or more non-whitespace characters
QUOTED_STRING := " , followed by any characters except unescaped " , followed by "
WS            := one or more whitespace characters
```

**Escaping:** not supported yet. A `"` cannot appear inside a `QUOTED_STRING`.

**Invalid input:** any line not matching one of the command rules above is rejected as an error — this includes an unknown command word, the wrong number of arguments, or an unterminated quote.

## Examples

```
SET name Great          -> OK
SET name "Great Ezenna" -> OK
GET name                -> Great Ezenna
GET "name"               -> Great Ezenna
DEL name                 -> OK
GET name                 -> NIL
DEL name                 -> NOT FOUND
```

## Responses

| Situation | Response |
|---|---|
| `SET` succeeds | `OK` |
| `GET` finds the key | the value |
| `GET` on a missing key | `NIL` |
| `DEL` on a present key | `OK` |
| `DEL` on a missing key | `NOT FOUND` |
| Malformed input | an error message (unknown command / wrong argument count / unterminated quote) |