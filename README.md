# [Resource Query Language](https://github.com/persvr/rql) parser library for Rust.

Work in progress.

## Quick start

```
# args
$ rql "and(eq(test,10),lt(foo,10),or(gt(bar,100),ne(bar,10)))"
And(
  [
    Filter(Eq, Identifier("test"), IntegerLiteral(10)),
    Filter(Lt, Identifier("foo"), IntegerLiteral(10)),
    Or(
      [
        Filter(Gt, Identifier("bar"), IntegerLiteral(100)),
        Filter(NotEq, Identifier("bar"), IntegerLiteral(10))
      ]
    )
  ]
)

# stdin
$ echo "and(eq(test,10),lt(foo,10),or(gt(bar,100),ne(bar,10)))" | rql -
And(
  [
    Filter(Eq, Identifier("test"), IntegerLiteral(10)),
    Filter(Lt, Identifier("foo"), IntegerLiteral(10)),
    Or(
      [
        Filter(Gt, Identifier("bar"), IntegerLiteral(100)),
        Filter(NotEq, Identifier("bar"), IntegerLiteral(10))
      ]
    )
  ]
)
```