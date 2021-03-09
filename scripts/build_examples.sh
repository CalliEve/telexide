#!/bin/bash

for f in examples/*
do
    cargo build --manifest-path  $f/Cargo.toml
done
