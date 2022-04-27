use clap::Parser;
use std::sync::Mutex;
use std::io::{Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use logger::logging;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8000)]
    port: u64,

    #[clap(short, long, default_value_t = 10000)]
    count: u64,
}

fn handle_close(tcp_stream: TcpStream) {
    tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn handle_connection(mut tcp_stream: TcpStream, count: u64) {
    let send_string = format!("{}\r\n", count.to_string());
    let send_bytes = send_string.as_bytes();
    match tcp_stream.write(&send_bytes[..send_bytes.len()]) {
        Result::Ok(_) => {
            tcp_stream.flush().unwrap();
            logging::info(format!("send: {}", count));
        },
        Result::Err(e) => {
            handle_close(tcp_stream);
            logging::error(e.to_string());
        }
    }
}

fn main() {
    let args = Args::parse();
    let addr = format!("0.0.0.0:{}", args.port.to_string());
    let _result = TcpListener::bind(&addr);

    if let Err(e) = _result {
        logging::error(format!("bind error: {}", &e));
        return;
    }
    let listener = _result.unwrap();
    logging::info(format!("server_start {}", &addr));

    let current_count = Mutex::new(0);
    for stream in listener.incoming() {
        if let Err(e) = stream {
            logging::error(e.to_string());
            break;
        }

        let _streams = stream.unwrap();

        let mut _current_count = current_count.lock().unwrap();
        *_current_count += 1;
        let count = _current_count.clone();
        if count > args.count {
            handle_close(_streams);
            break;
        }
        thread::spawn(move || {
            handle_connection(_streams, count);
        });
    }
    logging::info(format!("server_terminated."));
}
