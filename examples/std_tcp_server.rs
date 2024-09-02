use std::{
  net::{TcpListener, TcpStream},
  thread,
};

use emmapack::{PacketReadableSync, PacketSendableSync};

fn run_server() {
  let server = TcpListener::bind("0.0.0.0:1234").unwrap();
  let mut received_client = server.accept().unwrap().0;
  let msg = received_client.read_packet_sync::<String>().unwrap();
  println!("Message from client: {msg}");
}

fn run_client() {
  let mut client_connected_to_server = TcpStream::connect("127.0.0.1:1234").unwrap();
  client_connected_to_server
    .send_packet_sync(&"hello")
    .unwrap();
}

fn main() {
  let t1 = thread::spawn(run_server);
  let t2 = thread::spawn(run_client);

  t1.join().unwrap();
  t2.join().unwrap();
}
