use proto::{QuadClient, AddPointRequest, Point, DeletePointRequest};
use clap::{Parser, Subcommand, Args};
use tonic::transport::Channel;
use anyhow::{Result, anyhow};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    AddPoint(InputPoint),
    DeletePoint(InputPoint)
}

#[derive(Args)]
struct InputPoint {
    x: i64,
    y: i64
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddPoint(point) => {
            let request = tonic::Request::new(AddPointRequest{
                    point: Some(Point{
                        x: point.x,
                        y: point.y
                    })
                }
            );

            let mut client = get_client().await?;

            let _response = client.add_point(request)
                .await
                .map_err(|_x| anyhow!("Failure adding point"))?;
        },
        Commands::DeletePoint(point) => {
            let request = tonic::Request::new(DeletePointRequest{
                    point: Some(Point{
                        x: point.x,
                        y: point.y
                    })
                }
            );

            let mut client = get_client().await?;

            let _response = client.delete_point(request)
                .await
                .map_err(|_x| anyhow!("Failure deleting point"))?;
        }
    }

    Ok(())
}

async fn get_client() -> Result<QuadClient<Channel>> {
    let client = QuadClient::connect("http://[::1]:50051")
        .await
        .map_err(|_x| anyhow!("Cannot connect to service"))?;

    return Ok(client);
}

