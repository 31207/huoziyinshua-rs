#!/bin/bash

# This script checks the sample format of all .wav files in the ./sources directory.
# If the sample format is not s16 (16-bit signed integer),
# it converts the file to s16 using ffmpeg and replaces the original file.

for f in ./sources/*.wav; do
    fmt=$(ffprobe -v error -select_streams a:0 \
        -show_entries stream=sample_fmt \
        -of default=noprint_wrappers=1:nokey=1 "$f")

    if [ "$fmt" != "s16" ]; then
        echo "$f : $fmt"
        ffmpeg -i "$f" -c:a pcm_s16le "${f%.wav}_s16.wav"
        rm "$f"
        mv "${f%.wav}_s16.wav" "$f"
    fi
done