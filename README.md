# cookbook
A collection of delicious recipes for the Rust programming language.  Bon Appétit.

# Todo
A list of topics that are worth considering.  Some of these topics have been taking from the Python 3 cookbook.
  - Finding the Largest or Smallest N Items in an Slice, Iterator, or Range(?) object like a Set.
  - Implementing a Priority Queue (Constructing an iterator problem).
  - Implementing a mutable iterator.
  - Intersecting Hasmaps?  Union of Hashmaps?
  - Deduplicating items in an iterator.
  - Finding most requently occuring item in slice or general iterator.
  - Doing a "GroupBy" while iterating over product types.
  - Find common **borrowck** issues! StackExchange?
    - Borrowed `self` in `if {} else {}` clasue [SO ref](http://stackoverflow.com/questions/30243606/if-let-borrow-conundrum)
    - Mutating `self` after borrowing `self`. [SO ref](http://stackoverflow.com/questions/27335252/cannot-borrow-self)
    - Using lifetimes to control your borrows. [SO ref](http://stackoverflow.com/questions/32403837/mutable-borrow-seems-to-outlive-its-scope)
    - Good question, what's the morale of the story? [SO ref](http://stackoverflow.com/questions/30087338/why-does-unwrap-or-keep-borrow-in-scope)
    - When to clarify when you need `iter`, `into_iter`, or `iter_mut`? [SO ref](http://stackoverflow.com/questions/35298490/veciter-converts-to-borrow-of-option)
