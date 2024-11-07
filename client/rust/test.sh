#!/bin/sh
if [ "$#" -eq 0 ]; then
  RUST_LOG=shared=trace cargo test --package shared --lib -- --show-output
else
  #eg domain::lessons::tests::test_get_lessons_doesnt_persist_deleted_items
  RUST_LOG=shared=trace cargo test --package shared --lib -- "$1" --exact --show-output
fi
