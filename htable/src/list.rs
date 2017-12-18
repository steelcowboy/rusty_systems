//use std::cmp;
use std::cmp::Ordering;

#[derive(Default,Eq)]
pub struct CFPair {
    chr: char, 
    freq: usize,
    is_node: bool,
}

struct Link(Option<Box<Node>>);

pub struct Node {
    elem: CFPair,
    next: Link,
}

impl Link {
    pub fn new() -> Link {
        Link(None)
    }

    pub fn new_boxed(&self, n: Node) -> Link {
        Link(Some(Box::new(n)))
    }

    pub fn insert_sorted(self, n: CFPair) -> Link {
        match self.0 {
            None => self.new_boxed(Node {
                elem: n,
                next: Link(None),
            }),
            Some(curr) => {
                if curr.elem > n {
                    return self.new_boxed(Node {
                        elem: n,
                        next: Link(Some(curr)),
                    });
                }
                self.new_boxed(Node {
                    elem: curr.elem,
                    next: self.insert_sorted(n),
                })
            },
        }
    }    

}

impl PartialOrd for CFPair {
    fn partial_cmp(&self, other: &CFPair) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CFPair {
    fn cmp(&self, other: &CFPair) -> Ordering {
        if self.freq == other.freq {
            if self.is_node {
                return Ordering::Less;
            }
            if other.is_node {
                return Ordering::Greater;
            }
            return self.chr.cmp(&other.chr);
        }
        self.freq.cmp(&other.freq)
    }
}

impl PartialEq for CFPair {
    fn eq(&self, other: &CFPair) -> bool {
        self.chr == other.chr && 
            self.freq == other.freq &&
            self.is_node == other.is_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfpair_compare() {
        let cf1 = CFPair { 
            chr: 'a', 
            freq: 4, 
            is_node: false,
        };

        // Test same frequencies
        let cf2 = CFPair { 
            chr: 'b', 
            freq: 4, 
            is_node: false,
        };
        let cf3 = CFPair { 
            chr: 'd', 
            freq: 4, 
            is_node: true,
        };

        let cf4 = CFPair { 
            chr: 'e', 
            freq: 4, 
            is_node: false,
        };

        let cf5 = CFPair {
            chr: 'z',
            freq: 2,
            is_node: false,
        };

        assert!(cf1 < cf2);
        assert!(cf1 > cf3);
        assert!(cf1 < cf4);
        assert!(cf1 > cf5);
    }
}
