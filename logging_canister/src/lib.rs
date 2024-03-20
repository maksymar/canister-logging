fn set_hook() {
    std::panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_info = format!("Panicked at '{}', {}:{}:{}", msg, file, line, col);
        //ic_cdk::print(&err_info);  // Skip printing to avoid duplicated messages.
        ic_cdk::trap(&err_info);
    }));
}

#[ic_cdk::init]
fn init() {
    set_hook();
}

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
//     set_hook();
//     // ic_cdk::print("Message before heartbeat trap is preserved");
//     // ic_cdk::trap("Heartbeat trap");
//     ic_cdk::print("Message before heartbeat panic is preserved");
//     panic!("Heartbeat panic");
// }
