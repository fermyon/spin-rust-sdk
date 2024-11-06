#!/bin/bash

EXAMPLES_DIR="examples"

if [[ ! -d "$EXAMPLES_DIR" ]]; then
  echo "Directory $EXAMPLES_DIR does not exist."
  exit 1
fi

for example in "$EXAMPLES_DIR"/*; do
  if [[ -d "$example" ]]; then
    example_name=$(basename "$example")
    
    # tells GitHub Action to start a new collapsible log section
    echo "::group::Building example: $example_name"
    
    # cd into example app directory
    cd "$example" || { echo "Failed to change directory to $example"; continue; }
    
    build_output=$(spin build 2>&1)
    build_status=$?
    
    echo "$build_output"
    
    if [[ $build_status -eq 0 ]]; then
      echo "✅ spin build succeeded for $example_name"
    else
      echo "❌ spin build failed for $example_name"
    fi
    
    echo "::endgroup::"
    
    # return to the parent directory
    cd - >/dev/null
  fi
done
