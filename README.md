# canister-logging

```shell
$ dfx stop; dfx start --background --clean
$ dfx deploy

$ dfx canister call logging_canister greet Alice
$ dfx canister call logging_canister greet Bob
$ dfx canister call logging_canister explicit_trap ''
$ dfx canister call logging_canister explicit_trap 'it’s a trap!'
$ dfx canister call logging_canister explicit_panic
$ dfx canister call logging_canister failed_unwrap
$ dfx canister call logging_canister memory_oob

$ dfx canister logs logging_canister

$ dfx stop
```

Logs example

```shell
$ dfx canister logs logging_canister
[0. 2024-03-14T11:40:41.67773931Z]: Hello, Alice!
[1. 2024-03-14T11:40:46.461205193Z]: Hello, Bob!
[2. 2024-03-14T11:40:51.392137436Z]: Canister explicitly called trap without a message
[3. 2024-03-14T11:40:56.909091065Z]: Canister explicitly called trap: it’s a trap!
[4. 2024-03-14T11:41:01.124157144Z]: Panicked at 'panic attack', logging_canister/src/lib.rs:15:5
[5. 2024-03-14T11:41:01.124157144Z]: Canister explicitly called trap: Panicked at 'panic attack', logging_canister/src/lib.rs:15:5
[6. 2024-03-14T11:41:06.304553595Z]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', logging_canister/src/lib.rs:20:47
[7. 2024-03-14T11:41:06.304553595Z]: Canister explicitly called trap: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', logging_canister/src/lib.rs:20:47
[8. 2024-03-14T11:41:11.89746486Z]: Canister trapped: stable memory out of bounds
```
