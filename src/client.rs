use tokio_vsock::{VsockStream, VsockAddr, VMADDR_CID_LOCAL};
use echo::echo_client::EchoClient;
use echo::EchoRequest;
use hyper_util::rt::TokioIo;

use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

pub mod echo {
    tonic::include_proto!("echo");
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| async {
            let sockaddr = VsockAddr::new(VMADDR_CID_LOCAL, 1234);
            Ok::<_, std::io::Error>(TokioIo::new(VsockStream::connect(sockaddr).await?))
        }))
        .await?;

    let mut client = EchoClient::new(channel);

    let request = tonic::Request::new(EchoRequest {
        message: String::from("Hello, World!"),
    });

    let response = client.echo_string(request).await?;

    println!("Response: {}", response.into_inner().message);

    Ok(())
}
