#!/bin/bash
BINARIES=("pos2pot" "ndstat")

cargo build --release
for bin in ${BINARIES[@]};do
    echo "uploading $bin..."
    aws s3 cp target/release/$bin s3://mjhong-public/hanslab_utils/bin/$bin --acl public-read
done
