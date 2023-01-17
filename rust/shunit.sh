#!/bin/bash

testInOut() {
  for file in `ls ../cases/in*`
  do
    echo "${file/in/"out"}";
    RES=$(cat < ${file} | cargo run)
    EXP=$(cat "${file/in/"out"}")
    assertEquals "${EXP}" "${RES}"
  done;
}

# Load shUnit2.
. /usr/share/shunit2/shunit2