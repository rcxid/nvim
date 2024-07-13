#!/bin/bash

cargo build --release > /dev/null 2>&1
rm -rf lua/*.so

os_type=$(uname -s)

for file_path in $(pwd)/target/release/*; do
  file_name=$(basename $file_path)
  if [[ "$os_type" = "Linux" ]]; then
    flag=$(echo $file_name | grep '^lib.*\.so$')
    if [[ -n $flag ]]; then
      len=${#file_name}
      dest_path="lua/${file_name:3:$[len-3]}"
      ln -s $file_path $dest_path
    fi
  elif [[ "$os_type" = "Darwin" ]]; then
    flag=$(echo $file_name | grep '^lib.*\.dylib$')
    if [[ -n $flag ]]; then
      len=${#file_name}
      dest_path="lua/${file_name:3:$[len-9]}.so"
      ln -s $file_path $dest_path
    fi
  fi
done
