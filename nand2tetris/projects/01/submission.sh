#!/bin/bash

# Find all .hdl files in the current directory and its subdirectories
hdl_files=$(find . -name "*.hdl")

# Create a zip file named project1.zip
zip project1.zip ${hdl_files[@]}
