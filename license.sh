#!/bin/bash

# Path to the license header template
LICENSE_HEADER="license_header.txt"

# Loop through each .rs file in the src directory
for file in ./src/**/*rs; do
    # Check if the file already contains the license header (to avoid duplicates)
    if ! grep -q "License" "$file"; then
        # Add the license header to the top of the file
        cat "$LICENSE_HEADER" "$file" > temp && mv temp "$file"
        echo "Added license to $file"
    else
        echo "License already exists in $file"
    fi
done
