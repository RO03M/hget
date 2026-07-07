# Comments

This project distinguishes between two kinds of comments.

## User comments (`//`)

Lines beginning with `//` are treated as documentation intended for humans.

Example:

```http
// Retrieves the authenticated user
GET /me
```

These comments have no semantic meaning and are ignored by the parser.

## Disabled elements (`#`)

The `#` character disables language constructs such as:

- variables
- headers
- query parameters

Example:

```http
# Authorization: Bearer {{token}}
```

This is intentionally different from `//`.

If `#` were also treated as a generic comment, the parser could not distinguish between:

```http
# Authorization: Bearer {{token}}
```

(which is a disabled header)

and

```http
# This request requires authentication.
```

(which would be a normal comment).

Using `//` for documentation and `#` for disabled language elements makes the syntax unambiguous.