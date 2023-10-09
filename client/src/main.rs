use proto::{QuadClient, AddPointRequest, Circle, Point, DeletePointRequest, FindWithinRangeRequest};
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
    /// Adds a new point to the Quad Tree
    AddPoint(InputPoint),
    /// Deletes a point from the Quad Tree
    DeletePoint(InputPoint),
    /// Find all points within the specified circular region
    FindWithinRange(InputCircle),
    /// Returns all Quad Tree nodes
    GetAllQuads
}

#[derive(Args)]
struct InputPoint {
    /// x coorindate of the input point
    x: f64,
    /// y coordinate of the input point
    y: f64
}

#[derive(Args)]
struct InputCircle {
    x: f64,
    y: f64,
    radius: f64
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
            });

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
            });

            let mut client = get_client().await?;

            let _response = client.delete_point(request)
                .await
                .map_err(|_x| anyhow!("Failure deleting point"))?;
        },
        Commands::FindWithinRange(circle) => {
            let request = tonic::Request::new(FindWithinRangeRequest{
                circle: Some(Circle{
                    x: circle.x,
                    y: circle.y, 
                    radius: circle.radius 
                })
            });

            let mut client = get_client().await?;

            let response = client.find_within_range(request)
                .await
                .map_err(|_x| anyhow!("Failure finding within range"))?;

            let points = response.into_inner().points;

            for point in points {
                println!("{}, {}", point.x, point.y);
            }
        },
        Commands::GetAllQuads => {
            let request = tonic::Request::new(());

            let mut client = get_client().await?;

            let response = client.get_all_quads(request)
                .await
                .map_err(|_x| anyhow!("Failing getting all quads"))?;

            match response.into_inner().quad_node {
                Some(quad) => {
                    println!("{:#?}", quad);
                },
                None => {
                    println!("Cannot display quad");
                }
            }
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

