# canister-logging

```shell
$ dfx stop; dfx start --background --clean
$ dfx deploy

$ dfx canister call logging_canister greet Alice
$ dfx canister call logging_canister greet Bob
$ dfx canister call logging_canister explicit_trap oops!
$ dfx canister call logging_canister explicit_trap ''
$ dfx canister call logging_canister explicit_panic
$ dfx canister call logging_canister failed_unwrap
$ dfx canister call logging_canister memory_oob

$ dfx canister logs logging_canister

$ dfx stop
```

Logs example

```shell
$ dfx canister logs logging_canister
[0. 2024-03-18T13:47:50.532537542Z]: Hello, Alice!
[1. 2024-03-18T13:48:01.595812726Z]: Hello, Bob!
[2. 2024-03-18T13:48:08.310495273Z]: Canister explicitly called trap: oops!
[3. 2024-03-18T13:48:13.174937314Z]: Canister explicitly called trap without a message
[4. 2024-03-18T13:48:18.795510205Z]: Panicked at 'panic attack', logging_canister/src/lib.rs:15:5
[5. 2024-03-18T13:48:18.795510205Z]: Canister explicitly called trap: Panicked at 'panic attack', logging_canister/src/lib.rs:15:5
[6. 2024-03-18T13:48:23.301561192Z]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', logging_canister/src/lib.rs:26:47
[7. 2024-03-18T13:48:23.301561192Z]: Canister explicitly called trap: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', logging_canister/src/lib.rs:26:47
[8. 2024-03-18T13:48:27.648673456Z]: Canister trapped: stable memory out of bounds
```
