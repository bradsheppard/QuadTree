mod quad {
    tonic::include_proto!("quad");
}

pub use quad::{AddPointRequest, DeletePointRequest, Point};
pub use quad::quad_server::{Quad, QuadServer};
pub use quad::quad_client::QuadClient;

