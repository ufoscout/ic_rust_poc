
# Run all tests
[group('test')]
test test_name="": build
  cargo test {{test_name}} -- --nocapture
