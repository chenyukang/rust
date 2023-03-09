#!/bin/sh

RUST_BACKTRACE=1 $RUSTC src/lib.rs -Z treat-err-as-bug=1 1>$TMPDIR/rust-test-1.log 2>&1
RUST_BACKTRACE=full $RUSTC src/lib.rs -Z treat-err-as-bug=1 1>$TMPDIR/rust-test-2.log 2>&1

short=$(cat $TMPDIR/rust-test-1.log | wc -l)
full=$(cat $TMPDIR/rust-test-2.log | wc -l)
echo "short backtrace: $short"
echo "full  backtrace: $full"

if [ $full -gt $short ]; then
    exit 0
else
    exit 1
fi
