mod quad {
    tonic::include_proto!("quad");
}

pub use quad::{AddPointRequest, DeletePointRequest, Circle, FindWithinRangeRequest, FindWithinRangeResponse, GetAllQuadsResponse, Point, QuadNode};
pub use quad::quad_server::{Quad, QuadServer};
pub use quad::quad_client::QuadClient;

