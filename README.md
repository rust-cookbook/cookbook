# cookbook
A collection of delicious recipes for the Rust programming language.  Bon Appétit.

# Todo
A list of topics that are worth considering.  Some of these topics have been taken from the Python 3 Cookbook.
  - ** Iterators **
    - Finding the largest or smallest N Items in an Slice, Iterator, or Range(?) object like a Set.
    - Implementing a Priority Queue (Constructing an iterator problem).
    - Implementing a mutable iterator.
    - Intersecting Hasmaps?  Union of Hashmaps?
    - Deduplicating items in an iterator.
    - Finding most requently occuring item in slice or general iterator.
    - Doing a "GroupBy" while iterating over product types.
    - Expanding a list, ie: using `flat_map`, like `x.iter().flat_map(|&n| vec![n, n+1, n+2]).collect()`.
  - **borrowck** issues! D:
    - Borrowed `self` in `if {} else {}` clasue [SO ref](http://stackoverflow.com/questions/30243606/if-let-borrow-conundrum)
    - Mutating `self` after borrowing `self`. [SO ref](http://stackoverflow.com/questions/27335252/cannot-borrow-self)
    - Using lifetimes to control your borrows. [SO ref](http://stackoverflow.com/questions/32403837/mutable-borrow-seems-to-outlive-its-scope)
    - Good question, what's the morale of the story? [SO ref](http://stackoverflow.com/questions/30087338/why-does-unwrap-or-keep-borrow-in-scope)
    - When to clarify when you need `iter`, `into_iter`, or `iter_mut`? [SO ref](http://stackoverflow.com/questions/35298490/veciter-converts-to-borrow-of-option)
    - Coerce multiple iterators, and iterator proxy.  Ie, why doesn't this work:
      ```
      for y in if reverse { (0..4).rev() } else { 0..4 } { ..
      ```
      <https://is.gd/ScNcpy> -steveklabnik
  - **Closures Closures Closures!!**
    - Functions that return functions, see this [SO ref](http://stackoverflow.com/questions/27886474/recursive-function-type)
    - Get some good examples from this post [here](http://smallcultfollowing.com/babysteps/blog/2014/11/26/purging-proc/)
  - ** Uncategorized **
    - Caching function return values.  Hygiene?
    
