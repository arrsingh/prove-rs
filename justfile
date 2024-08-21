test $RUST_BACKTRACE="1":
     cargo test --package prove-rs api::tests:: -- --nocapture
