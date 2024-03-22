# canister-logging

```shell
# Window A.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ dfx stop; dfx start --clean
$ dfx stop

# Pane B1.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ dfx deploy

$ dfx canister call demo_canister print aaa
$ dfx canister call demo_canister print bbb
$ dfx canister call demo_canister trap oops!
$ dfx canister call demo_canister trap ''
$ dfx canister call demo_canister panic aaa!
$ dfx canister call demo_canister failed_unwrap
$ dfx canister call demo_canister memory_oob

$ dfx canister logs demo_canister

# Pane B2.
$ unalias dfx; alias dfx='./../sdk/target/debug/dfx'
$ ./stream_logs.sh
```

Logs example

```shell
[76. 2024-03-21T12:33:01.965292743Z]: right before timer trap
[77. 2024-03-21T12:33:01.965292743Z]: Canister explicitly called trap: timer trap
[78. 2024-03-21T12:33:05.450071111Z]: right before timer trap
[79. 2024-03-21T12:33:05.450071111Z]: Canister explicitly called trap: timer trap
...
[104. 2024-03-21T12:33:45.176316425Z]: right before heartbeat panic
[105. 2024-03-21T12:33:45.176316425Z]: Canister explicitly called trap: Panicked at 'heartbeat panic', demo_canister/src/lib.rs:72:5
[106. 2024-03-21T12:33:46.268548377Z]: right before heartbeat panic
[107. 2024-03-21T12:33:46.268548377Z]: Canister explicitly called trap: Panicked at 'heartbeat panic', demo_canister/src/lib.rs:72:5
[108. 2024-03-21T12:34:08.596444882Z]: aaa
[109. 2024-03-21T12:34:13.608001044Z]: bbb
[110. 2024-03-21T12:34:18.200893597Z]: right before trap
[111. 2024-03-21T12:34:18.200893597Z]: Canister explicitly called trap: oops!
[112. 2024-03-21T12:34:30.862245326Z]: right before trap
[113. 2024-03-21T12:34:30.862245326Z]: Canister explicitly called trap without a message
[114. 2024-03-21T12:34:35.234216006Z]: right before panic
[115. 2024-03-21T12:34:35.234216006Z]: Canister explicitly called trap: Panicked at 'aaa!', demo_canister/src/lib.rs:52:5
[116. 2024-03-21T12:34:40.529936619Z]: right before failed unwrap
[117. 2024-03-21T12:34:40.529936619Z]: Canister explicitly called trap: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', demo_canister/src/lib.rs:66:47
[118. 2024-03-21T12:34:46.356277571Z]: right before memory out of bounds
[119. 2024-03-21T12:34:46.356277571Z]: Canister trapped: stable memory out of bounds
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
# Split pane horisontally (one below another): Ctrl+B+"
# Navigate between panes: Ctrl+B+(arrows)
$ cd demo/canister-logging
```
