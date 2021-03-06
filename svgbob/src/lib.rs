//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(is_sorted)]
#![feature(test)]

pub mod buffer;
mod map;
mod point;
pub mod util;

pub use buffer::{
    fragment, fragment::Fragment, Cell, CellBuffer, Direction, FragmentBuffer, Property, Settings,
    Signal,
};
pub use point::Point;
pub use sauron::Node;

/// convert svgbob ascii art to svg
pub fn to_svg(ascii: &str) -> String {
    let cb = CellBuffer::from(ascii);
    let node: Node<()> = cb.get_node();
    node.to_string()
}

/// convert ascii art into an svg
pub fn to_svg_with_settings(ascii: &str, settings: &Settings) -> String {
    let cb = CellBuffer::from(ascii);
    let (node, _w, _h): (Node<()>, f32, f32) = cb.get_node_with_size(settings);
    node.to_string()
}
