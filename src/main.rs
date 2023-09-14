use mini_redis::{Connection, Frame};
use tokio::{net::{TcpListener, TcpStream}, spawn};


#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:9877").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        spawn(async move {
            process(socket).await
        });
    }
}

async fn process(socket: TcpStream) {

    
    let mut connent = Connection::new(socket);

    if let Some(frame) = connent.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connent.write_frame(&response).await.unwrap();
    }
}