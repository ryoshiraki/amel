pub mod circle;
pub mod ellipse;
pub mod polyline2d;
pub mod rectangle;
pub mod regular_polygon;
pub mod segment2d;
pub mod triangle2d;

pub use circle::*;
pub use ellipse::*;
pub use polyline2d::*;
pub use rectangle::*;
pub use regular_polygon::*;
pub use segment2d::*;
pub use triangle2d::*;

pub trait Primitive2d {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindingOrder {
    Clockwise,
    CounterClockwise,
    #[doc(alias = "Degenerate")]
    Invalid,
}

// #[derive(Clone, Debug)]
// pub enum Primitive {
//     // Arrow(Arrow),
//     Circle(Circle),
//     Ellipse(Ellipse),
//     Segment(Segment),
//     Line(Line),
//     // MeshVertexless(mesh::Vertexless),
//     // Mesh(Mesh),
//     // PathInit(PathInit),
//     // PathFill(PathFill),
//     // PathStroke(PathStroke),
//     // Path(Path),
//     // PolygonInit(PolygonInit),
//     // Polygon(Polygon),
//     // Quad(Quad),
//     Rectangle(Rectangle),
//     // Text(Text),
//     // Texture(Texture),
//     Triangle2d(Triangle2d),
// }
