// use std::panic;

// #[ic_cdk::init]
// fn init() {
//     panic::set_hook(Box::new(|_| {
//         // Do nothing to avoid duplicated panic messages.
//     }));
// }

#[ic_cdk::update]
fn greet(name: String) -> String {
    let message = format!("Hello, {}!", name);
    ic_cdk::print(message.clone());
    message
}

#[ic_cdk::update]
fn explicit_trap(message: String) {
    ic_cdk::print("Message before trap is preserved");
    ic_cdk::trap(&message);
}

#[ic_cdk::update]
fn explicit_panic() {
    ic_cdk::print("Message before panic is preserved");
    panic!("panic attack");
}

#[ic_cdk::update]
fn memory_oob() {
    ic_cdk::print("Message before memory oob is preserved");
    let mut buffer = vec![0u8; 10];
    ic_cdk::api::stable::stable_read(20, &mut buffer);
}

#[ic_cdk::update]
fn failed_unwrap() {
    ic_cdk::print("Message before failed unwrap is preserved");
    String::from_utf8(vec![0xc0, 0xff, 0xee]).unwrap();
}

// #[ic_cdk::heartbeat]
// fn heartbeat() {
//     ic_cdk::print("Message before heartbeat trap is preserved");
//     ic_cdk::trap("Heartbeat trap");
// }
