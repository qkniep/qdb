// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

const BTREE_ORDER: usize = 64;

struct Node {
    keys: Vec<Vec<u8>>,
    children: Vec<Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
