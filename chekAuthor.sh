#!/bin/bash

if [[ $1 == "@"]]
then 
    echo "Valid address";
    exit 0;
else 
    echo "Invalid address";
    exit 1;
fi    