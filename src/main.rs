fn main() {
    let nc = nats::Options::new()
        .error_callback(|err| println!("connection received an error: {}", err))
        .disconnect_callback(|| println!("connection has been lost"))
        .reconnect_callback(|| println!("connection has been reestablished"))
        .close_callback(|| println!("connection has been closed"))
        .lame_duck_callback(|| println!("server entered lame duck mode"))
        .connect("127.0.0.1")
        .unwrap();

    println!("connected");
    let js = nats::jetstream::new(nc);

    println!("subscribe");
    let sub = js.subscribe("test_subject").unwrap();
    println!("subscribed");

    while let Some(msg) = sub.next() {
        let t = &msg.data;
        let decoded_msg = String::from_utf8(t.to_vec()).unwrap();
        dbg!(&decoded_msg);

        msg.ack().unwrap();
    }
}
