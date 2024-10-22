#!/bin/bash

FILE=$1
z3 $FILE > res1 && target/release/parser $FILE > res2 && cmp res1 res2; echo $?