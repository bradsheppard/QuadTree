use std::sync::{Arc, RwLock};
use config::Config;
use tonic::{Request, Response, Status, transport::Server};
use storage::{Quad as InMemoryQuad, Point, Circle};
use proto::{AddPointRequest, GetAllQuadsResponse, DeletePointRequest, FindWithinRangeRequest, FindWithinRangeResponse, Quad, QuadServer, QuadNode};

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

    async fn find_within_range(&self, request: Request<FindWithinRangeRequest>) -> Result<Response<FindWithinRangeResponse>, Status> {
        let circle = request.into_inner().circle;

        match circle {
            Some(c) => {
                let circle = Circle{
                    center: Point { 
                        x: c.x,
                        y: c.y 
                    },
                    radius: c.radius
                };

                let quad = self.in_memory_quad.as_ref();
                let lock = quad.write();

                match lock {
                    Ok(value) => {
                        let points = value.find_within_range(&circle)
                            .iter()
                            .map(|p| proto::Point{x: p.x, y: p.y})
                            .collect();
                        let response = FindWithinRangeResponse { 
                            points
                        };
                        return Ok(Response::new(response));
                    }
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

    async fn get_all_quads(&self, _request: Request<()>) -> Result<Response<GetAllQuadsResponse>, Status> {
        let quad = self.in_memory_quad.as_ref();
        let lock = quad.read();

        match lock {
            Ok(value) => {
                let mut target_quad = proto::QuadNode{
                    top_left: None,
                    top_right: None,
                    bottom_left: None,
                    bottom_right: None,
                    is_child: false,
                    points: vec![]
                };
                self.recursive_search(&mut target_quad, &value);

                return Ok(Response::new(GetAllQuadsResponse{quad_node: Some(target_quad)}));
            }
            Err(e) => {
                println!("Error acquiring write lock {}", e);
                return Err(Status::internal("Internal Error"));
            }
        }
    }
}

impl QuadService {
    fn recursive_search(&self, target_quad: &mut QuadNode, source_quad: &InMemoryQuad) {
        for point in &source_quad.points {
            target_quad.points.push(proto::Point{
                x: point.x,
                y: point.y
            });
        }

        if source_quad.top_left_quad.is_some() {
            target_quad.top_left = Some(Box::new(QuadNode{
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
                points: vec![],
                is_child: true
            }));

            self.recursive_search(target_quad.top_left.as_mut().unwrap(), source_quad.top_left_quad.as_ref().unwrap());
        }

        if source_quad.top_right_quad.is_some() {
            target_quad.top_right = Some(Box::new(QuadNode{
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
                points: vec![],
                is_child: true
            }));

            self.recursive_search(target_quad.top_right.as_mut().unwrap(), source_quad.top_right_quad.as_ref().unwrap());
        }

        if source_quad.bottom_left_quad.is_some() {
            target_quad.bottom_left = Some(Box::new(QuadNode{
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
                points: vec![],
                is_child: true
            }));

            self.recursive_search(target_quad.bottom_left.as_mut().unwrap(), source_quad.bottom_left_quad.as_ref().unwrap());
        }

        if source_quad.bottom_right_quad.is_some() {
            target_quad.bottom_right = Some(Box::new(QuadNode{
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
                points: vec![],
                is_child: true
            }));

            self.recursive_search(target_quad.bottom_right.as_mut().unwrap(), source_quad.bottom_right_quad.as_ref().unwrap());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_config = Config::parse("/etc/quadtree/config.ini");
    let config = match parsed_config {
        Ok(r) => r,
        Err(_) => Config::default()
    };

    let port = config.port;

    let addr = format!("[::1]:{port}").parse().unwrap();

    let top_left = Point { 
        x: config.top_left_x,
        y: config.top_left_y
    };
    let bottom_right = Point {
        x: config.bottom_right_x,
        y: config.bottom_right_y
    };

    let quad = InMemoryQuad::from(top_left, bottom_right, config.capacity);

    println!("Quad server listening on {}", addr);

    let service = QuadService{
        in_memory_quad: Arc::new(RwLock::new(quad))
    };

    let server = QuadServer::new(service);

    Server::builder()
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}
