// fn first() {
//    let mut v = vec![1,2,3,4,5,6,7,8,9];

//    for num in &mut v {
//        if num < 5 {
//            v.push(num *num);
//        }

//        println!("{}",num);
//    }
// }


/// We now can iterate over v while adding more to v but
///   1. We don't have a mechanism to continue iterating over the newly
///      added elements, but we could do this with a queue, if need be.
///
///   2. We were required to add create a copy, which may or may not be good.
fn second() {
    let mut v = vec![5,2,3,7,9];
    
    let v_clone = v.clone();
    for num in v_clone {
        if num < 5 {
            v.push(num*num);
        }
        
        println!("{}", num);
    }

    println!("{:?}", v);
}


/// Iterating over indices

fn third() {
    let mut v = vec![5,2,3,7,9];
    
    let mut n = 0;
    while n < v.len() {
        let num = v[n];
        n += 1;
       
        if num < 5 {
            v.push(num*num);
        }

        println!("{}", num);
    }
}

/// Suppose you need to remove an element while iterating.
/// You can do the reverse swap-pop method. Do however note
/// that you will not preserve ordering while 

fn fourth() {
    let mut v = vec![5,2,3,7,9];

    for i in (0..v.len()).rev() {
        if v[i] < 5 {
            //swap and pop
            v.swap_remove(i);
            println!("Pop!");
            continue;
        }
        println!("{}", v[i]);
    }

    println!("{:?}", v);
}

/// Dealing with iterators that don't have indicies.  Let's take a string for instance
/// (even though you can save indicies using `char_indices()`).
/// Here you can save the items you wish to push to the end

fn fifth() {
    let mut my_string = "Hello, String!".to_string();
    let mut acc = String::new();
    
    for ch in my_string.chars() {
        if ch == 'l' {
            acc.push('!');
        }
    }
    my_string = my_string + &acc;
    println!("{}", my_string);

}


fn main() {
    // second();
    // third();
    fourth();
    fifth();
}