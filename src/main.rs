use std::env::current_dir;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const CONENT_LEN_HEAD: &str = "Content-Length:";

pub enum Response {
    Http {
        version: f64,
        status_code: u64,
        reason_phrase: String,
        headers: String,
        body: String,
        crlf: String,
    },
}

pub enum Request {
    Get { version: f64 },
}

struct InternetFmt;

impl InternetFmt {
    fn from_http_resp(resp: Response) -> String {
        match resp {
            Response::Http {
                version,
                status_code,
                reason_phrase,
                headers,
                body,
                crlf,
            } => {
                format!(
                    "HTTP/{} {} {}{}{} {}{}{}{}",
                    version,
                    status_code,
                    reason_phrase,
                    crlf,
                    headers,
                    body.len(),
                    crlf,
                    crlf,
                    body
                )
            }
        }
    }

    fn header_from_http_req(req: Request) -> String {
        match req {
            Request::Get { version } => format!("GET / HTTP/{}", version),
        }
    }
}

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    for stream in listener.incoming() {
        let mut tcp = stream.unwrap();

        let mut buf = [0; 1024];
        tcp.read(&mut buf).unwrap();

        let get_str = InternetFmt::header_from_http_req(Request::Get { version: 1.1 });
        let get = get_str.as_bytes();

        let (status, reason, html_file) = if buf.starts_with(get) {
            (200, "OK", "hello.html")
        } else {
            (404, "NOT FOUND", "404.html")
        };

        let curr_dir = current_dir().unwrap().join("html");
        let content= fs::read_to_string(curr_dir.join(html_file)).unwrap();
        let resp = InternetFmt::from_http_resp(Response::Http {
            version: 1.1,
            status_code: status,
            reason_phrase: reason.to_owned(),
            headers: CONENT_LEN_HEAD.to_owned(),
            body: String::from(&content),
            crlf: "\r\n".to_owned(),
        });

        tcp.write(resp.as_bytes()).unwrap();
        tcp.flush().unwrap();
    }
}
