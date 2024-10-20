#!/bin/bash

cargo aoc > /dev/null 2>&1 || cargo install cargo-aoc --locked

if [ "$(cargo aoc credentials)" = "Error: No session token available" ]; then
  echo "Enter AoC session cookie:"
  cookie=$(read)
  cargo aoc credentials "$cookie"
fi
