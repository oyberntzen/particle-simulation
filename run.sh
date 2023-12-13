#! /bin/bash

trap cleanup INT
function cleanup() {
    rm ./result/frames/*
    exit 1
}

cargo run
ffmpeg -framerate 30 -pattern_type glob -i "./result/frames/*.png" -c:a copy -shortest -c:v libx264 -pix_fmt yuv420p "./result/$1.mp4"
cleanup
