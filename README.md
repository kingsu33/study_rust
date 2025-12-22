# study_rust
cd C:\Users\User\개발\hello-rust
docker run --rm -it -v ${PWD}:/app -w /app rust:1.75 bash

# 도커 빌드 
docker build -f Dockerfile.dev -t rust-dev .

# docker run
docker run -it -v ${PWD}:/app rust-dev

# 처음 시작 시 
cargo init --bin

# 실행은 
cargo run

# 빌드는 
cargo build
