use clap::Parser;
use chrono::Local;
use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::io::{BufRead, BufReader};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value_t = 8000)]
    port: u64,
}

fn info(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S.%Z");
    let log = format!("[{0}] [INFO] {1}", now, msg);
    println!("{}", log);
}

fn error(msg: String) {
    let now = Local::now().format("%Y-%m-%d_%H:%M:%S.%Z");
    let log = format!("[{0}] [ERROR] {1}", now, msg);
    eprintln!("{}", log);
}

fn is_prime(num: u64)->bool {
    for current_num in 1..num {
        if current_num  == 1 {
            continue;
        }
        if current_num == num {
            continue;
        }
        if num % current_num == 0 {
            return false;
        }
    }
    return true
}

fn main() {
    let args = Args::parse();
    let addr = format!("{0}:{1}", args.host, args.port.to_string()).parse().unwrap();

    info(format!("client_start {}", &addr));
    loop {
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10));
        if let Err(e) = stream {
            error(format!("connection error: {}", &e));
            break;
        }

        let mut _stream = stream.unwrap();

        let result = _stream.set_read_timeout(Some(Duration::from_secs(5)));
        if let Err(e) = result {
            error(format!("connection error: {}", &e));
            break;
        }

        let mut reader = BufReader::new(_stream);
        let mut buffer = Vec::new();
        match reader.read_until(b'\n', &mut buffer) {
            Result::Ok(_) => {
                let recv = str::from_utf8(&buffer);
                if let Err(err) = recv {
                    error(err.to_string());
                    break;
                }
                let _recv = recv.unwrap();

                let number = _recv.parse::<u64>();
                if let Err(err) = number {
                    error(err.to_string());
                    break;
                }
                let _number = number.unwrap();

                let output = if is_prime(_number) {
                    format!("{}_is_prime", _recv)
                }else{
                    format!("{}_is_not_prime", _recv)
                };
                info(output);
            },
            Result::Err(e) => {
                error(e.to_string());
                break;
            }
        };
    }
    info(format!("client_terminated."));
}
