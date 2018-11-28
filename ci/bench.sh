if [[ "$TRAVIS_RUST_VERSION" == nightly ]]; then
    cargo bench
fi