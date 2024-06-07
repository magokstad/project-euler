use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use memoize::memoize;

fn main() {
    let map: Rc<RefCell<HashMap<(usize, usize), usize>>> = Rc::new(RefCell::new(HashMap::new()));
    println!("{}", find_paths(map, 20, 20));
    println!("{}", find_paths2(20, 20));
}


/// Manual memoization
fn find_paths(map: Rc<RefCell<HashMap<(usize, usize), usize>>>, l: usize, r: usize) -> usize {
    let mut paths = 0;

    let b = map.borrow();
    if b.contains_key(&(l, r)) {
        return *b.get(&(l, r)).unwrap();
    }
    drop(b);

    if l > 0 {
        paths += find_paths(map.clone(), l - 1, r);
    }
    if r > 0 {
        paths += find_paths(map.clone(), l, r - 1);
    }
    if r == 0 && l == 0 {
        paths += 1
    }

    let mut b = map.borrow_mut();
    b.insert((l, r), paths);

    paths
}

/// Crate-based memoization
#[memoize]
fn find_paths2(l: usize, r: usize) -> usize {
    let mut paths = 0;

    if l > 0 {
        paths += find_paths2(l - 1, r);
    }
    if r > 0 {
        paths += find_paths2(l, r - 1);
    }
    if r == 0 && l == 0 {
        paths += 1;
    }

    paths
}
