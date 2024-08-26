# Ownership

- Rust's ownership is unique

- Set of rules that govern memory management

- Rules are enforced at compile time

- If any of the rules are violated, the program won't compile

## Terminologies

- **Owner**: The owner of a value is the variable or data structure that
  holds it and is responsible for all allocating and freeing the memory
  used to store that data

- **Scope**: A range within a program for which an item is valid

## Rules of ownership

1. Each value in Rust has an owner

2. There can be only one woner at a time

3. When owner goes out of scope, the value will be dropped

## Types of Scope

1. **Global Scope**

   - Accessible throughout the entire program

2. **Local Scope**

   - Accessible only within particular function or block of code
   - Not accessible outside of that function or block
