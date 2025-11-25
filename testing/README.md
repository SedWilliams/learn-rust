# Testing and Test Driven Development (TDD)
* Write a test for new functionality that we know fails (initially).
* Write the minimum code necessary to make the test pass and the feature functional.
* Refactor the code while ensuring all tests still pass and the feature is still functional.
* Repeat the process for new features or improvements.

This is the procedure for Test Driven Development as outlined in the _Rust Programming Language_ book.

## What I learned writing This
---
* Started to understand Rust attributes and how to use them for testing.
    * Attributes in rust are a metaprogramming technique meaning they "write" new code for the thing referenced as having said attribute.
    * For example, the `#[test]` attribute tells the Rust compiler that the function it decorates is a test function. And the rust compiler will generate the necessary code to run that function as a test when the `cargo test` command is executed.
    * Or another example is the `#[derive]` attribute which automatically implements certain traits for a struct or enum.
* Learned how to write unit tests in Rust using the `#[test]` attribute.


