use tonic::{transport::Server, Request, Response, Status};
use tokio_vsock::{VsockListener, VsockAddr, VMADDR_CID_LOCAL};
use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Default)]
pub struct MyEcho {}

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn echo_string(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let reply = echo::EchoResponse {
            message: format!("{} - echoed from server", request.into_inner().message),
        };
        Ok(Response::new(reply))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let echo = MyEcho::default();

    let sockaddr = VsockAddr::new(VMADDR_CID_LOCAL, 1234);
    let vsock_listener = VsockListener::bind(sockaddr)?;

    Server::builder()
        .add_service(EchoServer::new(echo))
        .serve_with_incoming(vsock_listener.incoming())
        .await?;

    Ok(())
}
