# canister-logging

```shell
# Window A.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ dfx stop; dfx start --clean
$ dfx stop

# Pane B1.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ dfx deploy

$ dfx canister call logging_canister greet Alice
$ dfx canister call logging_canister greet Bob
$ dfx canister call logging_canister explicit_trap oops!
$ dfx canister call logging_canister explicit_trap ''
$ dfx canister call logging_canister explicit_panic
$ dfx canister call logging_canister failed_unwrap
$ dfx canister call logging_canister memory_oob

$ dfx canister logs logging_canister

# Pane B2.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ ./read_logs.sh
```

Logs example

```shell
[0. 2024-03-20T12:17:32.44713786Z]: Hello, Alice!
[1. 2024-03-20T12:17:49.066851935Z]: Hello, Bob!
[2. 2024-03-20T12:18:05.890427183Z]: Message before trap is preserved
[3. 2024-03-20T12:18:05.890427183Z]: Canister explicitly called trap: oops!
[4. 2024-03-20T12:18:23.067717178Z]: Message before trap is preserved
[5. 2024-03-20T12:18:23.067717178Z]: Canister explicitly called trap without a message
[6. 2024-03-20T12:18:40.661320818Z]: Message before panic is preserved
[7. 2024-03-20T12:18:40.661320818Z]: Canister explicitly called trap: Panicked at 'panic attack', logging_canister/src/lib.rs:38:5
[8. 2024-03-20T12:24:56.277509746Z]: Hello, ccc!
[9. 2024-03-20T12:25:12.105787237Z]: Message before panic is preserved
[10. 2024-03-20T12:25:12.105787237Z]: Canister explicitly called trap: Panicked at 'panic attack', logging_canister/src/lib.rs:38:5
[11. 2024-03-20T12:25:20.932478166Z]: Message before failed unwrap is preserved
[12. 2024-03-20T12:25:20.932478166Z]: Canister explicitly called trap: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', logging_canister/src/lib.rs:51:47
[13. 2024-03-20T12:25:31.096857436Z]: Message before memory oob is preserved
[14. 2024-03-20T12:25:31.096857436Z]: Canister trapped: stable memory out of bounds
```

## Preparation

```bash
$ cd ~
$ mkdir demo
$ cd demo

# [Before new DFX release]
# Checkout SDK repo.
$ git clone git@github.com:dfinity/sdk.git
$ cd sdk
# Update replica to log trap messages.
$ SHA=bb0ac2e51992bb237a9900c379bf15b7a2b0b97d  # Mar 12
$ ./scripts/update-replica.sh $SHA
# Build DFX with custom replica.
$ cargo build --bin dfx
$ ./target/debug/dfx -V
dfx 0.18.0+rev28.dirty-b0405bac
# Back to demo dir.
$ cd ..

# Checkout canister-logging demo repo.
$ git clone git@github.com:maksymar/canister-logging.git
$ cd canister-logging
# Alias new dfx.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ alias | grep dfx
dfx=./../sdk/target/debug/dfx
$ dfx -V
dfx 0.18.0+rev28.dirty-b0405bac

# Tmux shortcuts
# Create new window: Ctrl+B+c
# Split pane vertically (next to each other): Ctrl+B+%
# Navigate between panes: Ctrl+B+(arrows)
# Split pane horisontally (one below another): Ctrl+B+"
$ cd demo/canister-logging
```
