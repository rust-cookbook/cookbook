# Mutating Iterators

A naive approach for appending elements to an iterator while iterating won't quite work in rust.  There are a few reasons for this.  For one, during an iteration you are taking a reference from the iterator.  Once you have a refence to an element of an iterator, you therefor have a reference to the iterator itself, and so you aren't allowed to modify the iterator since it's being reference.  So what could go wrong?  Is the borrow checker just being overly strict here?  Maybe, but there are some potential problems with trying to modify the iterator while you are holding a reference.  Let's say you iterator is a vector called `my_vec`, and you have a refence to an element in your vector called `item`.  If you modify `my_vec` during the iteration, it may cause `my_vec` to reallocate space to make room for new items.  At this point `item` would then point to an old memory location which is no longer valid, creating a hanging pointer.  But that's not to say that you _can't_ modify iterators while iterating through them; it just means you have to use a little care.

## Adding Elements While Iterating

This first example shows how you can add elements to an iterator while iterating.  The general idea is to maintain a reserve vector to add your new elements and then appending to the iterator when you are done.

```rust
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
```

For such a simple example, you may find a functional approach better.  In this example, we use `filter_map` to replace the characters we want, and then  `chain` the iterators together.  

```rust
fn iter_and_append_f()  
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
```

## Removing Elements While Iterating

Suppose on the otherhand that you need to remove elements while iterating.  This example shows you a relatively efficient method.  I like to call this method the __swap-pop__ method.  In this example we are able to modify the iterator while iterating since we are accessing the iterating through indices and not taking references to the elements.  So, while this may be safe, as in indices never have the dangling point problem we discussed before, you will need to be careful to prevent out-of-bound access.

```rust
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
```

As before, you may find such a simple example easier to handle using `filter` (I certainly would recommend this), but it has the potential of not being as efficient as the code shown before.  It's probably better to leave optimizations after you have done proper benchmarks of your program in a production like environment anyway.

```rust
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
```

## Passing a Reference

```rust
trait Ring {
    fn is_prime(self) -> bool;
}

impl Ring for u8 {
    fn is_prime(self) -> bool {
        if self < 2 {
            return false
        }
        
        for n in 2..self {
            if self % n == 0 {
                return false
            }
        }
        
        true
    }
}

fn remove_primes(v: &Vec<u8>) -> Vec<u8> {
    v.iter().filter(|&u| !u.is_prime()).cloned().collect::<Vec<u8>>()
}
```