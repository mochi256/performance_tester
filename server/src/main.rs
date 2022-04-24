use clap::Parser;
use std::sync::Mutex;
use chrono::Local;
use std::io::{Write};
use std::net::TcpListener;
use std::net::{TcpStream, SocketAddr};
use std::thread;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8000)]
    port: u64,

    #[clap(short, long, default_value_t = 10000)]
    count: u64,
}

fn info(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S");
    let log = format!("[{0}] [INFO] {1}", now, msg);
    println!("{}", log);
}

fn error(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S");
    let log = format!("[{0}] [ERROR] {1}", now, msg);
    eprintln!("{}", log);
}

fn handle_connection(stream: (TcpStream, SocketAddr), count: u64) {
    let mut tcp_stream: TcpStream = stream.0;
    let send_string = format!("{}\r\n", count.to_string());
    let send_bytes = send_string.as_bytes();
    match tcp_stream.write(&send_bytes[..send_bytes.len()]) {
        Result::Ok(_) => {
            info(format!("response: {}", count));
            tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
        },
        Result::Err(e) => {
            error(e.to_string());
        }
    }
}

fn main() {
    let args = Args::parse();
    let addr = format!("0.0.0.0:{}", args.port.to_string());
    let _result = TcpListener::bind(&addr);

    if let Err(e) = _result {
        error(format!("bind error: {}", &e));
        return;
    }
    let listener = _result.unwrap();
    info(format!("server_start {}", &addr));

    let current_count = Mutex::new(0);
    loop {

        let stream = listener.accept();
        if let Err(e) = stream {
            error(e.to_string());
            continue;
        }

        let _streams = stream.unwrap();

        let mut _current_count = current_count.lock().unwrap();
        *_current_count += 1;
        let count = _current_count.clone();
        thread::spawn(move || {
            handle_connection(_streams, count);
        });
        if count >= args.count {
            break;
        }
        
    }
}
