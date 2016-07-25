#![allow(dead_code)]

// # Mutable Iterators


//
// We will implement a simple tree that could, say, represent a simple DOM
// structure.  You may notice that we don't actually allow for text in our
// simple representation, but this isn't important here, what we really want
// to investigate is how we would implement a mutable iterator using our custom
// data structure.  Our DOM structure will only allow for four different types
// of nodes: Header, Paragraph, Text, and Images.

enum NodeType {
    Header,
    Paragraph,
    Text,
    Image,
}

// Each node with have a list of `children` nodes and a tag for the respective
// DOM element type.

struct DomNode {
    children: Vec<DomNode>,
    node_type: NodeType
}

// To implement an iterator, we essentially just need to tell Rust two things:
//   a. What kind of elements will we be iteratorating over?  In this example
//      we will be iteratorating over the nodes of our Dom, so we will want
//      to iterate over `NodeType`s.
//   b. What is the next element?
//
// We will therefore have the responsibility of keeping track of where we are at
// while iteratorating over our DOM.  We will do this by constructing what is
// oftern called an "iterator interface".  We make one like this:

struct DomMutIterator<'a> {
    data: &'a mut DomNode,
    cursor: usize,
}

// We provide answers for the questions posted above for Rust by implementing an
// `Iterator` for our iterator interface.

impl<'a> Iterator for DomMutIterator<'a> {
    type Item = &'a mut NodeType;
    fn next(&mut self) -> Option<&'a mut NodeType> {
        
        // This part will require some thought...
        
        unimplemented!();
    }
}

fn main() { 
}

// Note: Rust implementation of creating a mutable iterator uses a `Range` object which, keeps track of the size of the vector.
// this allows for unchecked boundes [see here](https://doc.rust-lang.org/src/collections/up/src/libcollections/btree/map.rs.html#920).
// we should leave this as an exercise to the reader and show them the source to reference. They use the `Range` object to call the
// recurse in the vector [see here](https://doc.rust-lang.org/src/collections/up/src/libcollections/btree/map.rs.html#920).  Perhaps
// this might be a good idea for us as well.
