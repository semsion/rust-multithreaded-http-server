## Motivation

This repository was created as a learning tool to comprehend some fundamental Rust methodologies in terms of parallel operations, and creating a HTTP server; which consists of a functioning multithreaded HTTP server, providing a lightweight front end for sending requests to the server.

It was formed by working through project steps of 'The Rust Programming Language' book, with some minor adjustments, added unit tests for the thread pool, and added comments / documentation for public / non-public structs and functions. 

## Getting Started (steps to run the server) 

You must have Rust installed on your system for the server to run:

- Installing rustup on [Linux or macOS](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos)
- Installing rustup on [Windows](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows)
- Troubleshooting: https://doc.rust-lang.org/book/ch01-01-installation.html#troubleshooting

Once you have Rust installed:
- Clone this repository.
- Inside the root of the repository
  - Running `cargo check` will check that the code is compiling correctly.
  - Inline tests for the ThreadPool can be executed by running `cargo test`.
  - Start the server by running `cargo run`.
  - Then in a web browser, navigate to http://217.0.0.1:7878 and you should see the served HTML with the text 'Rust multithreaded HTTP server'.
  - Refreshing this page multiple times will send consecutive requests to the server, which you can view in the terminal.
  - Opening http://217.0.0.1:7878/sleep in another tab, and refresing this page will run a thread on the server which takes 5 seconds to complete, so while this request is waiting to respond to the server, and you refresh the first tab multiple times, in the terminal you will see that the first tab utilises other threads taken from the thread pool, and does not wait for the 'sleeping' thread to finish. A maximum of 4 parallel threads can be utilised at the same time.


Generated documentation can be viewed by running `cargo doc --open`

The book project details can be viewed [here](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
