#!/bin/bash

for f in examples/*
do
    cargo build --verbose --manifest-path  $f/Cargo.toml
done