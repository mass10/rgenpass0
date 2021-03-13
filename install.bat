@SETLOCAL

@CALL cargo fmt
@CALL cargo build --release
@CALL cargo install --path .
