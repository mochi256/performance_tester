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
    count_max: u64,

    #[clap(short, long, default_value_t = 100)]
    range: u64,
}

fn handle_close(tcp_stream: TcpStream) {
    tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn handle_connection(mut tcp_stream: TcpStream, count: u64, range: u64) {
    let mut send_string:String = "jobs:".to_string();
    for q in count..(count + range) {
        send_string = format!("{}{},", send_string, q.to_string());
    }
    send_string.pop();
    send_string = format!("{}\r\n", send_string);

    let send_bytes = send_string.as_bytes();
    match tcp_stream.write(&send_bytes[..send_bytes.len()]) {
        Result::Ok(_) => {
            tcp_stream.flush().unwrap();
            logging::info(format!("send: {0}-{1}", count, (count + range - 1)));
        },
        Result::Err(e) => {
            handle_close(tcp_stream);
            logging::error(e.to_string());
        }
    }
}

fn main() {
    let args = Args::parse();
    if args.count_max % args.range != 0 {
        logging::error(format!(
            "args error: {0} is not divisible by {1}", args.count_max, args.range
        ));
        return;
    }
    let addr = format!("0.0.0.0:{}", args.port.to_string());
    let _result = TcpListener::bind(&addr);

    if let Err(e) = _result {
        logging::error(format!("bind error: {}", &e));
        return;
    }
    let listener = _result.unwrap();
    logging::info(format!("server_start {}", &addr));

    let current_count = Mutex::new(1);
    for stream in listener.incoming() {
        if let Err(e) = stream {
            logging::error(e.to_string());
            break;
        }
        let _streams = stream.unwrap();

        let mut _current_count = current_count.lock().unwrap();
        let count = _current_count.clone();
        thread::spawn(move || {
            handle_connection(_streams, count, args.range);
        }).join().unwrap();
        if count + args.range > args.count_max {
            break;
        }
        *_current_count += args.range;
    }
    logging::info(format!("server_terminated."));
}
