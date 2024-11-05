#!/bin/sh
#RUST_LOG=shared=trace cargo test
#RUST_LOG=shared=trace cargo test --package shared --lib -- domain::auth::tests::test_login_success --exact --show-output
RUST_LOG=shared=trace cargo test --package shared --lib -- domain::lessons::tests::test_get_lessons_doesnt_persist_deleted_items --exact --show-output

