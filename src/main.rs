use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use viz::{server::conn::http1, Request, Responder, Result, Router, Tree};

async fn index(_: Request) -> Result<&'static str> {
    Ok("pindash.io")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {addr}");

    let app = Router::new().get("/", index);
    let tree = Arc::new(Tree::from(app));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tree = tree.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, Responder::new(tree, Some(addr)))
                .await
            {
                eprintln!("Error while serving HTTP connection: {err}");
            }
        });
    }
}
