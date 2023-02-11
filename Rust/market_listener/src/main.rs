use websocket::{ClientBuilder, Message};

fn main() {
    let mut client = ClientBuilder::new("wss://ws-api.binance.com/ws-api/v3")
        .unwrap()
        .connect(None)
        .unwrap();
    let message = Message::text("{\"id\":1,\"method\":\"time\"}");
    client.send_message(&message).unwrap();
    let c = client.recv_message().unwrap();
    println!("{:?}", c);
}
