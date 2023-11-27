# rust-lambda-function
AWS Lambda function written in Rust

## Debug
    cargo lambda watch

## Test
    http://localhost:9000/?name=chris

## Production Build
    cargo lambda build --release

If using AWS Graviton2 on Lambda, add the --arm64 flag to compile the code for ARM CPUs.

    cargo lambda build --release --arm64

Compress to zip file

    cargo lambda build --release --output-format zip
