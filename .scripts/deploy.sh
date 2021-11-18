
#!/bin/bash

set -v

./build.sh

dest="../ivanceras.github.io/madnotes/"

mkdir -p "$dest"

cp -r client/index.html client/assets client/pkg "$dest"


## Remove the ignore file on the pkg directory
rm $dest/pkg/.gitignore

