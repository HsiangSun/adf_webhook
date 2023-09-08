# Azure Data Factory Webhook

Welcome to Azure Data Factory Webhook, a lightweight webhook written in Rust and deployed with Docker. This project leverages Rust's efficiency and cross-compilation capabilities to produce a Docker image with a minimal footprint (only 21MB).

## Features
- Built with Rust for efficiency and performance.
- Dockerized deployment for easy scalability and management.
- Cross-compiled to the x86_64-unknown-linux-musl target for a minimal image size.
- Thanks to the Rust community for their dedication to this project.

## Getting Started
To get started with this webhook, follow these simple steps:

1. **Clone the Repository:** 
  ```sh
  git clone https://github.com/HsiangSun/adf_webhook.git
  cd adf_webhook
  ```
2. **Clone the Repository:**
  ```sh
  docker build -t adf_webhook .
  ```
3. **Run the Docker Container:**
  ```sh
  docker run -p <local_port>:<container_port> -v <local_env_path>:/app/.env adf_webhook
  ```
4. **Access Your Webhook:**
  ```
  Your webhook will be accessible at http://localhost:<local_port>. Make sure to configure your Azure Data Factory to use this webhook URL.
  ```

## License
This project is licensed under the [MIT License](https://chat.openai.com/c/LICENSE) - see the LICENSE file for details.


## Ref
* [Axum](https://github.com/tokio-rs/axum)
* [Axum-auth](https://github.com/Owez/axum-auth)
* [How to handle cross compile error](https://github.com/sfackler/rust-openssl/issues/980#issuecomment-415757400)


### Enjoy using Azure Data Factory Webhook, brought to you by Rust and Docker.
