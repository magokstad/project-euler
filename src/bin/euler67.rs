use std::collections::HashMap;
use lazy_static::lazy_static;
use memoize::memoize;
use crate::euler18::{Node, read_triangle};

mod euler18;

lazy_static! {
    static ref MAP: HashMap<(usize, usize), Node> = read_triangle(include_str!("../data/euler67.txt"));
}

fn main() {
    println!("{}", get_largest_sum(MAP[&(0, 0)]));
}

#[memoize]
pub fn get_largest_sum(node: Node) -> usize {
    let mut sum = node.val;

    let left = match node.left {
        Some(l) => get_largest_sum(MAP[&l]),
        None => 0
    };

    let right = match node.right {
        Some(r) => get_largest_sum(MAP[&r]),
        None => 0
    };

    if left > right {
        sum += left
    } else {
        sum += right;
    }

    sum
}
