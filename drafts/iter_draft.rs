/// We now can iterate over v while adding more to v but
///   1. We don't have a mechanism to continue iterating over the newly
///      added elements, but we could do this with a queue, if need be.
///
///   2. We were required to add create a copy, which may or may not be good.
fn iter_and_append() {
    let mut my_string = "Hello, String!".to_string();

    // Allocate a buffer
    let mut res = String::new();
    for ch in my_string.chars() {
        if ch == 'l' {
            res.push('!');
        }
    }

    my_string = my_string + &res;
    println!("{}", my_string);
}

/// For such a small example, you may find a functional approach better.
/// Which you choose is mostly up to personal preference.  Differences between
/// the functinoal approach and using a loop start to become noticable once
/// you need to modify 'global state'.  We will discuss that shortly, but
/// here is the first example, rewritten using `chain` with a `filter_map`.

fn iter_and_append_f() {
    let mut my_string = "Hello, String!".to_string();
    let mut new_string = String::new();

    new_string = my_string.chars().chain(
        my_string.chars().filter_map(
            |c| match c {
                'l' => Some('!'),
                _ => None
            }
        )
    ).collect();

    println!("{}", new_string);
}

/// Suppose you need to remove an element while iterating.
/// You can do the reverse swap-pop method. Do however note
/// that you will not preserve ordering while.
/// This also only works for iterators that give you
/// access to indices.

fn remove_evens() {
    // We iterate backwards for a few reaons:
    //   1. When we swap, we swap with an element
    //      that we have already processed.
    //   2. Pop is more efficient than removing
    //      an element from the beginning
    //   3. Popping reduces `v.len()` as we iterate
    //      which would require for use to compensate
    //      the index `i` after every removal if we
    //      iterated from 0..v.len();
    let mut v = vec![5,2,3,6,9];

    for i in (0..v.len()).rev() {
        if v[i] % 2 == 0 {
            v.swap_remove(i);
        }
    }

    println!("{:?}", v);
}

/// As before, you may find such a simple example easier to
/// handle using `filter` (I certainly would recommend this).

fn remove_vowels() {
    let mut my_string = "Hello, String!".to_string();
    let mut res = String::new();

    res = my_string.chars().filter(
        |&c| match c {
            'a' | 'e' | 'i' | 'o' | 'u'
               => false,
            _  => true,
        }
    ).collect();

    println!("{}", res);
}

fn main() {
    iter_and_append();
    iter_and_append_f();
    remove_evens();
    remove_vowels();
}