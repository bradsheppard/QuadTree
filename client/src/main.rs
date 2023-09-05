use proto::{QuadClient, AddPointRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = QuadClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AddPointRequest{
        point: None
    });

    let _response = client.add_point(request);

    println!("Added point");

    Ok(())
}
