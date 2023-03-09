#!/bin/sh

RUST_BACKTRACE=1 $RUSTC src/lib.rs -Z treat-err-as-bug=1 1>$TMPDIR/rust-test-1.log 2>&1
RUST_BACKTRACE=full $RUSTC src/lib.rs -Z treat-err-as-bug=1 1>$TMPDIR/rust-test-2.log 2>&1

short=$(cat $TMPDIR/rust-test-1.log | wc -l)
full=$(cat $TMPDIR/rust-test-2.log | wc -l)
rustc_query_count=$(cat $TMPDIR/rust-test-1.log | grep rustc_query_ | wc -l)
rustc_query_count_full=$(cat $TMPDIR/rust-test-2.log | grep rustc_query_ | wc -l)

cat $TMPDIR/rust-test-1.log
echo "====================="
cat $TMPDIR/rust-test-2.log

echo "short backtrace: $short"
echo "full  backtrace: $full"
echo "rustc_query_count: $rustc_query_count"
echo "rustc_query_count_full: $rustc_query_count_full"

## check `rustc_query_count` to avoid to missing `__rust_end_short_backtrace`
##  1 <= $rustc_query_count < $rustc_query_count_full
##  $rustc_query_count_full > 10
if [ $full -gt $short ] &&
    [ $rustc_query_count -gt 1 ] &&
    [ $rustc_query_count -lt $rustc_query_count_full ] &&
    [ $rustc_query_count_full -gt 10 ]; then
    exit 0
else
    exit 1
fi
