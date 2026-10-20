# Async HTTP GET Request in Rust

A simple Rust program that demonstrates how to make an asynchronous HTTP GET request using **Reqwest** and **Tokio**, with error handling using `error-chain`.

## Features

- Sends an asynchronous HTTP GET request.
- Uses Tokio to execute asynchronous code.
- Displays the HTTP response status code.
- Prints response headers in a readable format.
- Reads and displays the response body.
- Handles errors using the `?` operator and `error-chain`.

## Technologies Used

- **Rust** — Programming language.
- **Reqwest** — HTTP client for sending requests.
- **Tokio** — Asynchronous runtime.
- **Error Chain** — Error handling and propagation.

## Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
reqwest = { version = "0.13", features = ["json"] }
error-chain = "0.12.4"
```

## How It Works

1. **Initialize Tokio:** The `#[tokio::main]` macro sets up the runtime required to execute asynchronous code.
2. **Send a GET request:** `reqwest::get()` sends a request to `http://httpbin.org/get`.
3. **Await the response:** `.await` allows the asynchronous request to make progress until the response is available.
4. **Display status and headers:** The program prints the HTTP status code and response headers.
5. **Read the response body:** `response.text().await?` asynchronously reads the body and converts it into a `String`.
6. **Handle errors:** The `?` operator propagates errors instead of requiring explicit error checks at every step.

## How to Run

**1. Clone the repository**

```bash
git clone https://github.com/dewdropx/async-await.git
```

**2. Run the program**

```bash
cd async-await
cargo run
```

Cargo downloads the required dependencies, compiles the program, and executes it.

## Example Output

```text
Status: 200 OK
Headers:
{
    ...
}
Body:
{
  "args": {},
  "headers": {
    ...
  },
  "url": "http://httpbin.org/get"
}
```

The exact headers and response body may vary.

## Limitations

- Requires an active internet connection.
- Depends on the availability of the `httpbin.org` service.
