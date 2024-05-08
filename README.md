# My Custom Rust HTTP Server

## Description

This project is a custom-built HTTP server written entirely in Rust. It is designed to offer high performance and reliability, leveraging Rust's safety features and concurrency capabilities. This server can handle multiple client connections simultaneously and is ideal for applications requiring high throughput and low latency. Ofcoarse, this server is not intended for production use, but rather as a learning exercise or a starting point for building a more advanced HTTP server.
## Features

- **Fast and efficient processing**: Built on async I/O to handle multiple connections with minimal overhead.
- **Lightweight**: Minimal dependencies, focusing on the Rust standard library and a few essential crates.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust
- Cargo

You can install Rust and Cargo using [rustup](https://rustup.rs/).

### Installing

Clone the repository:

```bash
git clone https://github.com/GyroZepelix/rust-basic-http-server
cd rust-basic-http-server 
```

Build the project:

```bash
cargo build
```

Run the server:

```bash
cargo run
```

## Usage

Here is a simple example of how to use this HTTP server in your Rust application:

```rust
fn main() {
    let http_server = HttpServer::builder()
        .listener("127.0.0.1:4221")
        .add_route(RouteHandle::new(GET, "/", |cx| HttpStatusCode::Ok.into()))
        .add_route(RouteHandle::new(GET, "/secret", |cx| HttpStatusCode::Forbidden.into()))
        .add_route(RouteHandle::new(GET, "/echo/{to_echo}", echo))
        .build();

    http_server
        .run()
        .join()
        .unwrap();
}

fn echo(cx: &RequestContext) -> HttpResponse {
    let to_echo = cx.path_variables.get("to_echo")
        .map_or("".to_string(), |var| var.to_string());

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(&to_echo)
        .build()
}
```

Replace the handling logic with your own application's needs.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

- **GyroZepelix** - *Initial work* - [GyroZepelix](https://github.com/GyroZepelix)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

- Special thanks to CodeCrafters for providing the challenge that inspired this project.

