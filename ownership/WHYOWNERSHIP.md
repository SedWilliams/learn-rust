
# Why Ownership?

Say we want to store some input data in a variable.

This data...
- Cannot have a predetermined size at compile time as it is user input.
- Must be stored on the heap since it can vary in type and size.
- Must have a way to be __allocated, referenced, and deallocated__ (this goes for any prog. lang.)

__The third requirement is where Rust's specific implementation of the Ownership system comes into play__

Having the predetermined rules for allocating, who can reference, and deallocating stops many memory errors from happening.
- Double free errors
- Null pointer errors
- memory leaks (unfreed memory)

