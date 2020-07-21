# Shared Future Example

This example shows how to use a shared future to `await` on the same future from multiple tasks

To run, clone the repository then run:

```bash
cargo run
```

Output:

```bash
[src/main.rs:130] &all = [
    SpecialData {
        key: "Some key",
        data: f1a70768-3249-4933-b2b9-52a923ef5e76,
    },
    SpecialData {
        key: "Some key",
        data: f1a70768-3249-4933-b2b9-52a923ef5e76,
    },
    SpecialData {
        key: "Some other key",
        data: 63028cae-78bf-40f8-89d9-bacf38d8859f,
    },
    SpecialData {
        key: "Some other key",
        data: 63028cae-78bf-40f8-89d9-bacf38d8859f,
    },
    SpecialData {
        key: "Some key",
        data: f1a70768-3249-4933-b2b9-52a923ef5e76,
    },
    SpecialData {
        key: "Some other key",
        data: 63028cae-78bf-40f8-89d9-bacf38d8859f,
    },
    SpecialData {
        key: "Some other key",
        data: 63028cae-78bf-40f8-89d9-bacf38d8859f,
    },
    SpecialData {
        key: "Some other key",
        data: 63028cae-78bf-40f8-89d9-bacf38d8859f,
    },
]
[src/main.rs:131] &spy = [
    "Request from client Some key, returning f1a70768-3249-4933-b2b9-52a923ef5e76",
    "Request from client Some other key, returning 63028cae-78bf-40f8-89d9-bacf38d8859f",
]

```

