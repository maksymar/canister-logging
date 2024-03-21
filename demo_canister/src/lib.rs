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

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    set_hook();
}

#[ic_cdk::update]
fn print(text: String) {
    ic_cdk::print(text);
}

#[ic_cdk::update]
fn trap(message: String) {
    ic_cdk::print("Message before trap");
    ic_cdk::trap(&message);
}

#[ic_cdk::update]
fn panic(message: String) {
    ic_cdk::print("Message before panic");
    panic!("{}", message);
}

#[ic_cdk::update]
fn memory_oob() {
    ic_cdk::print("Message before memory out of bounds");
    const BUFFER_SIZE: u32 = 10;
    let mut buffer = vec![0u8; BUFFER_SIZE as usize];
    ic_cdk::api::stable::stable_read(BUFFER_SIZE + 1, &mut buffer);
}

#[ic_cdk::update]
fn failed_unwrap() {
    ic_cdk::print("Message before failed unwrap");
    String::from_utf8(vec![0xc0, 0xff, 0xee]).unwrap();
}

// #[ic_cdk::heartbeat]
// fn heartbeat() {
//     // ic_cdk::print("Message before heartbeat trap");
//     // ic_cdk::trap("Heartbeat trap");
//     ic_cdk::print("Message before heartbeat panic");
//     panic!("Heartbeat panic");
// }
