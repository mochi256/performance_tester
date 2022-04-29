use chrono::Local;

pub fn info(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S.%f");
    let log = format!("[{0}] [INFO] {1}", now, msg);
    println!("{}", log);
}

pub fn error(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S.%f");
    let log = format!("[{0}] [ERROR] {1}", now, msg);
    eprintln!("{}", log);
}
