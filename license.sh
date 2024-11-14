#!/bin/bash

LICENSE_HEADER="./license-header.rs"
for file in ./src/*rs; do
    if ! grep -q "License" "$file"; then
        cat "$LICENSE_HEADER" "$file" #> temp && mv temp "$file"
        echo "Added license to $file"
    else
        echo "License already exists in $file"
    fi
done
