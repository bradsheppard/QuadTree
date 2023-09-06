mod error;

use proto::{QuadClient, AddPointRequest, Point};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = std::env::args().nth(1);
    let point = std::env::args().nth(2);

    match (command, point) {
        (Some(c), Some(p)) => {
            let result = process_command(c.as_str(), p.as_str()).await;
            
            match result {
                Ok(_) => {
                    println!("Executed successfully");
                }
                Err(e) => {
                    println!("Error processing command {}", e);
                }
            }
        }
        _ => {
            println!("Unknown command");
        }
    }

    Ok(())
}

async fn process_command(command: &str, point: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = QuadClient::connect("http://[::1]:50051").await;

    match client {
        Ok(mut c) => process(&mut c, command, point).await,
        Err(e) => Err(Box::new(error::ErrorStatusError::new(e.to_string().as_str())))
    }

}

async fn process(client: &mut QuadClient<Channel>, command: &str, point: &str) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "addpoint" => {
            let request = tonic::Request::new(AddPointRequest{
                    point: Some(Point{
                        x: 1,
                        y: 2
                    })
                }
            );

            let response = client.add_point(request).await;

            match response {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    Err(Box::new(error::ErrorStatusError::new(e.message())))
                }
            }
        }
        _ => {
            Err(Box::new(error::InvalidCommandError))
        }
    }
}
