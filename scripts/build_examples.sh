#!/bin/bash

for f in examples/*
do
    cargo check --manifest-path  $f/Cargo.toml
done
