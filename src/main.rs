use std::{net::{TcpListener, TcpStream, IpAddr}, fs, io::Write};

fn main() {
    let ip: &str = "192.168.29.103";
    let port: &str = ":8080";
    let mut auto_count = 0;
    
    let mut addr: String = ip.to_owned();
    addr.push_str(port);

    let listener = TcpListener::bind(addr)
    .unwrap();

    for stream in listener.incoming() {
        auto_count += 1;
        match stream {
            Ok(stream) => handle_conn(stream, auto_count),
            Err(why) => panic!("{why}"),
        }
    }
}

#[derive(Debug)]
struct IPStruct {
    ip: IpAddr,
    count: i32,
}

fn handle_conn(mut stream: TcpStream, count_metrics: i32) {
    let mut many = vec![];
    let mut length = vec![];
    many.push(stream.peer_addr().unwrap().ip());

    for prev_ip_addr in many {
        println!("{}", prev_ip_addr);
        if prev_ip_addr == stream.peer_addr().unwrap().ip() {
            let what = IPStruct {
                ip: prev_ip_addr,
                count: count_metrics,
            };
            length.push(..what);
        } else {
            let what = IPStruct {
                ip: prev_ip_addr,
                count: 1,
            };
            length.push(..what);
        }
        for each in length.iter().map(|f| f.end.ip) {
            let local_iter = length.iter().map(|f| {
                if f.end.ip == prev_ip_addr {
                    return f.end.count
                } else {
                    0
                }
            });
            if each == prev_ip_addr {
                local_iter.for_each(|f| println!("IP: {:#?} created {:#?} requests", each, f));
            }
        }
    }
    // Print host IP and port
    println!("Host IP: {}, port: {}",
    stream.local_addr().unwrap().ip(),
    stream.local_addr().unwrap().port());

    // Print peer IP and port
    println!("Peer IP: {}, port: {}",
    stream.peer_addr().unwrap().ip(),
    stream.peer_addr().unwrap().port());

    let http_status = "HTTP/1.1 200 OK";
    let file = "/home/rilysh/torweb/server/target/debug/index.html";
    let file_str = String::from(file);
    let content = content_pub(&file_str);
    let length= content_pub(&file_str).len();
    let response = format!("{http_status}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn content_pub(which: &String) -> String {
    let file: String = fs::read_to_string(which)
    .unwrap();
    file
}
