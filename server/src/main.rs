use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status, transport::Server};
use storage::{Quad as InMemoryQuad, Point};
use proto::{AddPointRequest, DeletePointRequest, Quad, QuadServer};

#[derive(Debug, Default)]
pub struct QuadService {
    in_memory_quad: Arc<RwLock<InMemoryQuad>>
}

#[tonic::async_trait]
impl Quad for QuadService {
    async fn add_point(&self, request: Request<AddPointRequest>) -> Result<Response<()>, Status> {
        let point = request.into_inner().point;

        match point {
            Some(p) => {
                let point = Point{
                    x: p.x,
                    y: p.y
                };

                let quad = self.in_memory_quad.as_ref();
                let lock = quad.write();

                match lock {
                    Ok(mut value) => {
                        value.insert(&point);
                        return Ok(Response::new(()));
                    },
                    Err(e) => {
                        println!("Error acquiring write lock {}", e);
                        return Err(Status::internal("Internal Error"));
                    }
                }
            }
            None => {
                println!("Invalid input");
                return Err(Status::invalid_argument("Invalid input"));
            }
        }
    }

    async fn delete_point(&self, request: Request<DeletePointRequest>) -> Result<Response<()>, Status> {
        let point = request.into_inner().point;

        match point {
            Some(p) => {
                let point = Point{
                    x: p.x,
                    y: p.y
                };

                let quad = self.in_memory_quad.as_ref();
                let lock = quad.write();

                match lock {
                    Ok(mut value) => {
                        value.delete(&point);
                        return Ok(Response::new(()));
                    },
                    Err(e) => {
                        println!("Error acquiring write lock {}", e);
                        return Err(Status::internal("Internal Error"));
                    }
                }
            },
            None => {
                println!("Invalid input");
                return Err(Status::invalid_argument("Invalid input"));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Quad server listening on {}", addr);

    let service = QuadService{
        in_memory_quad: Arc::new(RwLock::new(InMemoryQuad::new()))
    };

    let server = QuadServer::new(service);

    Server::builder()
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}
