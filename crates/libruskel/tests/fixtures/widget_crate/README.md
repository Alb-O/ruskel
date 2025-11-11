To regenerate the rustdoc.json fixture for the widget_crate tests:

```sh
cd crates/libruskel/tests/fixtures/widget_crate && cargo rustdoc -Z unstable-options --output-format json -- --document-private-items
cp target/doc/widget_crate.json rustdoc.json
```