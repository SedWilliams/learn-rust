# IO practice project for learning Rust

This is a practice project from the book "Learning Rust" by Paul Johnson. 
The project implements rust features from previous chapters, focusing on 
input/output (IO) operations.

The goal of the project is to create a grep clone in the command line using Rust.

## What is grep?
---
GREP (*G*lobally search a *R*eg*E*xp and *P*rint) - is a command line utility that 
allows users to search for specific patterns within files or input streams.

## Install
---

## Prerequisites
---

## What I learned/problems I faced
---

Case 1: indexing vectors
    * Context: At the beginning of my program I wanted to make a struct
      type ArgsFormat that held the values of the program arguments
      type: Vec<String>, and I ran into multiple issues here. First, 
      I was struggling with the ownership model and borrow checker and
      getting it to be fine with how I got the args data into an 
      instantiated struct. After truggling with this I landed on the 
      following code:

        ``` Rust
        use std::env;

        //type defined for expected command line args
        #[derive(Debug)]
        struct ArgsFormat {
            args: String,
        }

        //instantiate a struct populated with the command line arguments
        fn build_args(mut args: Vec<String>) -> ArgsFormat {
            ArgsFormat {
                args: args.remove(0),
            }
        }

        fn main() {
            let args: ArgsFormat = build_args(env::args().collect());
            dbg!(args);
        }

        ```

    * Fix explained:
        * Flow: at main(), the args passed to the program are collected then 
          'moved' as a collection to build_args(). 
          build_args() accepts the collection of args and makes it mutable in the 
          function signature allowing the function to make the .remove() method call (mutation).

          This is necessary because in rust,
          *indexing is an attempt to take ownership* and rust does not
          allow that to happen if the value/collection is not mutable or movable. This
          is where I ran into a bunch of errors previously. I attempted
          to take ownership through mutation of an indexed value from a non-copy type (Vec<String>)
          , and it was also immutable (both distinct errors one is ownership and one is borrowing).
          *Although the above snippet could use some serious refactoring and debugging*,
          it allowed me to pass type checking and borrowing which was a struggle already.























