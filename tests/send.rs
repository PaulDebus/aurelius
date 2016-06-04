extern crate aurelius;
extern crate websocket;
extern crate url;

use websocket::{Client, Message, Receiver};
use url::Url;

use aurelius::Server;

#[test]
fn simple() {
    let server = Server::new();

    let websocket_port = server.websocket_addr().unwrap().port();

    let url = Url::parse(&format!("ws://localhost:{}", websocket_port)).unwrap();

    let request = Client::connect(url).unwrap();
    let response = request.send().unwrap();

    response.validate().unwrap();

    let (_, mut receiver) = response.begin().split();
    server.render_markdown("Hello, world!");

    let message: Message = receiver.incoming_messages().next().unwrap().unwrap();
    let html: String = String::from_utf8(message.payload.into_owned()).unwrap();
    assert_eq!(html.trim(), String::from("<p>Hello, world!</p>"));
}
