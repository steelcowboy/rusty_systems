use std::cmp;
use std::cmp::Ordering;

// This will set the bools to false and the nums to 0
#[derive(Default)]
pub struct Node<T> {
    is_leaf: bool,
    is_node: bool,

    chr: char,
    freq: usize, 

    left: Option,
    right: Option,
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        return self.is_leaf;
    }

    pub fn is_node(&self) -> bool {
        return self.is_node;
    }

    pub fn new_leaf(&mut self, chr: char, freq: usize) -> Node {
        self.is_leaf = true;
        self.chr = chr;
        self.freq = freq;
    }

    pub fn new_node(&mut self, l: &Node, r: &Node) -> Node {
        self.is_node = true;
        self.chr = cmp::max(l.chr, r.chr);
        self.freq = l.freq + r.freq;
    }

}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        if self.charcount == other.charcount {
            if self.is_node {
                return Ordering::Less;
            }
            if other.is_node {
                return Ordering::Greater;
            }
            return self.character.cmp(&other.character);
        }
        self.charcount.cmp(other.charcount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    ;
}
