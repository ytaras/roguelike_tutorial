if [[ "$TRAVIS_RUST_VERSION" == nightly ]]; then
      cargo tarpaulin --ciserver travis-ci --coveralls $TRAVIS_JOB_ID
      cargo tarpaulin --out Xml
      bash <(curl -s https://codecov.io/bash)
fi