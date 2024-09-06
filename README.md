# nvee

nvee is a simple config format for organizing moderately nested environment variables.

nvee uses syntax based on [TOML](https://toml.io), but is semantically simpler, since it's intended to be parsed as a flat key-value map instead of a full data structure.

## for example...
```toml
# .nvee

top_level = 123

[table]
another_key = "foo"
lucky_number = 7
```
this nvee document will be expanded to the following environment variables:

```sh
TOP_LEVEL=123
TABLE_ANOTHER_KEY=foo
TABLE_LUCKY_NUMBER=7
```

additionally, the nvee parser can interpret filenames as variable prefixes:

```toml
# example.nvee

top_level = 123
# ...snip...
```
```sh
EXAMPLE_TOP_LEVEL=123
EXAMPLE_TABLE_ANOTHER_KEY=foo
EXAMPLE_TABLE_LUCKY_NUMBER=7
```

## toml types

nvee currently natively supports TOML's string and integer data types. however, as environment variables have no type validation, other data can be represented ad-hoc as strings. while native support may expand in the future, there is currently no plan to support arrays, as lists and numbered keys are problematic in the environment.
