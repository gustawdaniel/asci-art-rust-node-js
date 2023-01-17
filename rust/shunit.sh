#! /bin/sh

testInOut() {
  for file in `ls ../cases/in*`
  do
    RES=$(cat < ${file} | cargo run)
    echo "${file/in/"out"}";
    EXP=$(cat "${file/in/"out"}")
    assertEquals "${EXP}" "${RES}"
  done;
}

# Load shUnit2.
. /usr/share/shunit2/shunit2