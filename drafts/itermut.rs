#![allow(dead_code)]

// # Mutable Iterators

// ## Part 1

// To start out our introduction to constructing mutable iterators, we will use
// a simple data structure consisting nothing but a vector of integers.  But
// these are our integers, so they are special, which is why we will now call
// them out `SpecialIntegers` type.

struct SpecialIntegers(Vec<u32>);

// To implement an iterator, we essentially need to tell Rust two things:
//   a. What kind of elements will we be iterating over.  When iterating
//      over our structure, we wil want our special integers (which are `u32`).
//   b. What is the next element when we are iterating.
//
// We will therefore have the responsibility of keeping track of where we are at
// while iterating.  We will do this by constructing what is
// oftern called an "iterator interface".  To keep track of our iteration, we will
// need our `SpecialIntegers` data structure and a cursor to keep track of where
// we are.

struct SpecialIntegersIterator<'a> {
    data: &'a mut SpecialIntegers,
    cursor: usize,
}

// We now need to tell rust how to get the next element in our iterator, which
// we accomplish by implementing the `Iterator` trait on our interator inteface.
// There is a problem that we will run into however.  Our iterator interface
// has a mutable borrow of our `SpecialIntegers` data structure.  So while we
// are holding this mutable borrow, we aren't technically allowed to give a mutable
// reference to elements in our list.  We therefore will be required to use unsafe.
// But before we do so, we should stop and think for a second: is what I am doing
// actually safe?

impl<'a> Iterator for &'a mut SpecialIntegersIterator<'a> {
    // The type of object we will be iterating over
    type Item = &'a mut u32;
    fn next(&mut self) -> Option<&'a mut u32> {
        unimplemented!();
    }
}

// However, before we will be able to use this iterator interface, unless you 
// are planning on creating a brand new `SpecialIntegersIterator` structure
// each you want to use an iterator, we need to tell Rust how to make this structure
// for wehenever we need an iterator.  We do this by implementing the `IntoIterator`
// trait.  Which, less, tells rust how to make an iterator interface by calling
// `iter_mut`.

impl<'a> IntoIterator for &'a mut SpecialIntegers {
    type Item =  &'a mut u32;
    type IntoIter = SpecialIntegers;
    
    fn into_iter(&mut self) -> &'a mut SpecialIntegersIterator<'a> {
        unimplemented!();
    }
}

// ## Part 2

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
