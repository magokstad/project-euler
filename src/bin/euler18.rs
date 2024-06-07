use std::collections::HashMap;
use lazy_static::lazy_static;
use memoize::memoize;

lazy_static! {
    static ref MAP: HashMap<(usize, usize), Node> = read_triangle(include_str!("../data/euler18.txt"));
}

fn main() {
    println!("{}", get_largest_sum(MAP[&(0, 0)]));
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Node {
    pub val: usize,
    pub left: Option<(usize, usize)>,
    pub right: Option<(usize, usize)>,
}

pub fn read_triangle(str: &'static str) -> HashMap<(usize, usize), Node> {
    let layers = str.split("\n").into_iter().map(|x: &str| x.split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut map = HashMap::new();
    for (li, layer) in layers.iter().enumerate() {
        for (ni, number) in layer.iter().enumerate() {
            let node = Node {
                val: number.parse().unwrap(),
                left: if li + 1 >= layers.len() { None } else { Some((li + 1, ni)) },
                right: if li + 1 >= layers.len() { None } else { Some((li + 1, ni + 1)) },
            };
            map.insert((li, ni), node);
        }
    }

    map
}

#[memoize]
fn get_largest_sum(node: Node) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::read_triangle;

    #[test]
    fn test_read_triangle_structure() {
        let str = include_str!("../data/euler18.txt");
        let map = read_triangle(str);

        assert!(map.get(&(0, 0)).is_some());
        assert_eq!(map.get(&(0, 0)).unwrap().val, 75);

        assert!(map.get(&(6, 0)).is_some());
        assert_eq!(map.get(&(6, 0)).unwrap().val, 88);

        assert!(map.get(&(8, 4)).is_some());
        assert_eq!(map.get(&(8, 4)).unwrap().val, 83);

        assert!(map.get(&(15, 0)).is_none());
    }

    #[test]
    fn test_read_triangle_links() {
        let str = include_str!("../data/euler18.txt");
        let map = read_triangle(str);

        let mut node = map.get(&(0, 0)).unwrap();
        for _ in 0..4 {
            node = map.get(&node.left.unwrap()).unwrap();
        }
        for _ in 0..4 {
            node = map.get(&node.right.unwrap()).unwrap();
        }

        assert_eq!(node.val, 83);
        assert_eq!(node.left, Some((9, 4)));
        assert_eq!(node.right, Some((9, 5)));
    }
}