# Rust Practice Repository

## Introduction to Rust

Rust is a systems programming language focused on safety, speed, and concurrency. It ensures memory safety without needing a garbage collector and is widely used for system-level applications, web development, and embedded systems. Rust provides powerful features like pattern matching, ownership, and type safety, making it a great choice for both beginners and experienced developers.

## Why Learn Rust?
- Ensures memory safety without garbage collection.
- Provides high performance for system-level programming.
- Enables safe concurrency and prevents data races.
- Used in web development (via WebAssembly), embedded systems, and high-performance computing.
- Actively supported by a strong open-source community.

For an interactive introduction to Rust, visit [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/).

## Concepts Covered
This repository contains Rust programs covering fundamental to advanced concepts. Below are the key topics covered:

### 1. Algorithms
- **Searching Algorithms:** Binary Search, Linear Search
- **Sorting Algorithms:** Bubble Sort, Insertion Sort, Merge Sort, Quick Sort, Selection Sort
- **Graph Algorithms:** Kruskal’s Algorithm, Prim’s Algorithm
- **Miscellaneous:** Fractional Knapsack (Greedy Algorithm), Min-Max Algorithm, Sudoku Board Validation

### 2. Data Structures
- **Stack** (Array-based Implementation)
- **Linear Queue and Circular Queue**

### 3. Rust-Specific Features
- Ownership and Borrowing
- Structs and Access Specifiers
- Traits and Generics
- Match Statements and Pattern Matching
- Error Handling using `Result`
- Iterators and Slices
- Option and Range Matching

### 4. Simple Algorithms
- Array Insertion
- Digit Count
- Pascal’s Triangle

## Setting Up a Local Rust Development Environment

### Install Rust and Cargo
Rust uses `cargo`, its package manager and build system. To install Rust and Cargo:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Follow the on-screen instructions, then restart your terminal and verify installation:
```sh
rustc --version
cargo --version
```

### Running Rust Programs
To compile and run a Rust program:
```sh
rustc filename.rs
./filename
```

Using Cargo for better dependency management:
```sh
cargo new project_name
cd project_name
cargo run
```

## How to Use This Repository

### Clone the Repository:
```sh
git clone https://github.com/NithinS74/Rust-practice-code.git
cd Rust-practice-code
```

### Run Example Programs
Navigate to the appropriate folder and execute Rust files using:
```sh
rustc filename.rs
./filename
```
Or use Cargo for Rust projects:
```sh
cargo run --bin filename
```

## Contributing
Contributions are welcome! Feel free to fork the repository and submit pull requests with improvements or new Rust examples.

## License
This repository is open-source and available under the [MIT License](LICENSE).

For more information on Rust, visit the [official Rust documentation](https://doc.rust-lang.org/).
