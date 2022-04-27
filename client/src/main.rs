use clap::Parser;
use std::net::{TcpStream};
use std::str;
use std::time::Duration;
use std::io::{BufRead, BufReader};
use logger::logging;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value_t = 8000)]
    port: u64,
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

    logging::info(format!("client_start {}", &addr));
    loop {
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10));
        if let Err(e) = stream {
            logging::error(format!("connection error: {}", &e));
            break;
        }
        let mut _stream = stream.unwrap();

        let result = _stream.set_read_timeout(Some(Duration::from_secs(5)));
        if let Err(e) = result {
            logging::error(format!("connection error: {}", &e));
            break;
        }

        let mut reader = BufReader::new(&_stream);
        let mut buffer = Vec::new();
        match reader.read_until(b'\n', &mut buffer) {
            Result::Ok(_) => {
                let recv = str::from_utf8(&buffer);
                if let Err(err) = recv {
                    logging::error(err.to_string());
                    break;
                }
                let _recv = recv.unwrap();
                
                let number_re = Regex::new(r"^(\d+)").unwrap();
                let number_caps = number_re.captures(_recv);
                if let None = number_caps {
                    logging::error(format!("number not found."));
                    break;
                }
                let _number_caps = number_caps.unwrap();

                let raw_number = &_number_caps[0];
                let number = raw_number.parse::<u64>();
                if let Err(err) = number {
                    logging::error(err.to_string());
                    break;
                }
                let _number = number.unwrap();

                let output = match is_prime(_number) {
                    true => format!("{}_is_prime", raw_number),
                    false => format!("{}_is_not_prime", raw_number)
                };
                logging::info(output);
            },
            Result::Err(e) => {
                logging::error(e.to_string());
                break;
            }
        };
    }
    logging::info(format!("client_terminated."));
}
