extern crate hyper;

use std::io;
use hyper::client::Client;
use hyper::server::{Server, Request, Response};
// use hyper::status::StatusCode;



fn main() {
    Server::http("0.0.0.0:8080").unwrap().handle(|req: Request, res: Response| {
         let client = Client::new();
         let mut rapi_resp = client.get("http://172.17.0.1/1kb").send().unwrap();

        // assert_eq!(adwapi_resp.status, hyper::Ok);

        // io::copy(&mut req, &mut res.start().unwrap()).unwrap();
        // println!("response: {}", adwapi_resp);
        io::copy(&mut rapi_resp, &mut res.start().unwrap()).unwrap();
        //res.send(b"Hello World!").unwrap();




    }).unwrap();
    
}
