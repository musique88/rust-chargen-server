use clap::Parser;
use rand::Rng;
use std::io::prelude::*;
use std::net::{TcpListener, UdpSocket};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 19)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    let sendstr = (' '..'~').map(|b| String::from(b)).reduce(|mut a, b| {a.push_str(b.as_str()); a}).unwrap();
    
    let tcpstr = sendstr.clone();
    let tcpthread = std::thread::spawn(move || {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", cli.port)).expect(&format!("Cannot bind tcp 0.0.0.0:{}", cli.port)[..]);
        println!("Listening on tcp port {}", cli.port);
        for stream in listener.incoming() {
            let tcpstr = tcpstr.clone();
            std::thread::spawn(move || {
                match stream {
                    Err(_) => {}
                    Ok(mut client) => {
                        loop {
                            match client
                                .write(format!("{}", tcpstr.clone()).as_bytes()) {
                                Ok(_) => {},
                                Err(_) => {continue}
                            }
                        }
                    }
                };
            });
        }
    });

    let udpstr = sendstr.clone();
    let udpthread = std::thread::spawn(move || {
        let listener = UdpSocket::bind(format!("0.0.0.0:{}", cli.port)).expect(&format!("Cannot bind udp 0.0.0.0:{}", cli.port)[..]);
        println!("Listening on udp port {}", cli.port);
        let mut buf = [0; 10];
        loop {
            let src_addr = match listener.recv_from(&mut buf) {
                Err(_) => {continue},
                Ok((_, addr)) => addr
            };

            let mut rng = rand::thread_rng();
            let mut stringtosend = String::from("");
            for i in 0..rng.gen_range(0..512) {
                stringtosend.push(udpstr.chars().nth(i % udpstr.len()).unwrap());
            }
            
            match listener
                .send_to(
                    format!("{}", stringtosend).as_bytes(),
                    src_addr,
                ) {
                Err(_) => {},
                Ok(_) => {}
            }
        }
    });
    tcpthread.join().unwrap();
    udpthread.join().unwrap();
}
