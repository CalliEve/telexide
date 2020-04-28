#!/bin/bash

suff=rs
for f in examples/*.rs
do
    IFS='/' read -ra fileArr <<< "$f"
    cargo build --verbose --example ${fileArr[1]%.$suff}
done