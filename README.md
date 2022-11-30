This is an example mock implementation of derive macros for the `ToLua` and `FromLua` traits
from the `mlua` crate. The actual macros are in [`mlua-derive`](mlua-derive), the [`src`](src) 
folder contains an example lua module showcasing the macros.

The easiest way to run the example is by pasting this command into your terminal:
```sh
cargo build -q && cp ./target/debug/libmoonbind.so moonbind.so && lua main.lua
```

Example output:
```
Getting data: 
table: 0x565463e79400
Key 'boxed_str' has type 'string'
Key 'description' has type 'string'
Key 'int_bytes' has type 'table'
Key 'double' has type 'number'
Key 'uint' has type 'number'
Key 'maybe' has type 'table'
Key 'int' has type 'number'
Key 'data' has type 'table'
Key 'float' has type 'number'
Key 'byte' has type 'number'
Key 'is_cool' has type 'boolean'
Key 'authors' has type 'table'
Key 'size' has type 'number'
Got data: 
Megastruct {
    is_cool: true,
    description: "This is a MEGASTRUCT because IT'S HUGE",
    boxed_str: "why not?",
    byte: 255,
    int: -2147483648,
    uint: 4294967295,
    size: 9223372036854775807,
    float: 3.1415927,
    double: 3.141592653589793,
    int_bytes: [
        239,
        190,
        173,
        222,
    ],
    data: [
        1,
        2,
        3,
        4,
    ],
    maybe: None,
    authors: {
        "The Rust Programming Language": "Steve Klabnik, Carol Nichols",
        "Republic": "Plato",
        "All Quiet on the Western Front": "Erich Maria Remarque",
    },
}
```