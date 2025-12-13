// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

mod coord;
mod line;

pub use coord::Coord as HomogeneousCoord;
pub use coord::PointAtInfinity;
pub use line::Line as HomogeneousLine;
pub use line::Slope;
