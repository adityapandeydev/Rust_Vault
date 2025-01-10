# RustVault

**RustVault** is a database project built using the Rust programming language. This project serves as an educational tool to learn Rust deeply, exploring both the foundational aspects of database design and the strengths of Rust's memory safety, performance, and concurrency features.

## Purpose

The primary goal of this project is to build a custom database system that handles storage, querying, and basic transaction management. It will provide hands-on experience with Rust's powerful features, including ownership, borrowing, concurrency, and error handling.

In addition to the core database functionality, there are plans to implement a Graphical User Interface (GUI) to interact with the database, making it more user-friendly and accessible.

## 🚀 Features Planned

- **Custom Storage Engine**: A lightweight storage solution for data persistence.
- **Query Processing**: Support for basic SQL-like queries (e.g., SELECT, INSERT).
- **Transaction Management**: Ensuring atomicity, consistency, isolation, and durability (ACID).
- **GUI Integration** (Future Plan): A user interface for easier interaction with the database.

## 🎯 Learning Objectives

- Understand and apply Rust's ownership model, memory safety, and concurrency.
- Design and implement a custom storage system.
- Learn about query parsing and execution.
- Build and optimize a robust, production-quality database system.
- Experiment with integrating a GUI.

## 🔧 Technologies Used

- **Programming Language**: Rust
- **Concurrency**: Rust's async/await, Threads, and Channels
- **Networking**: HTTP APIs (Using `hyper` or `warp` crates)
- **Database**: Custom storage engine, with plans for persistence via `PostgreSQL` or `Redis`
- **Version Control**: Git