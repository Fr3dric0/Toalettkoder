#!/bin/bash

set -e
set -o pipefail

cd list-locations-v1
cargo lambda build --release --arm64 --output-format zip

cd ../infra
cdk deploy --profile=linio