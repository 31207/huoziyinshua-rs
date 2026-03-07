for f in ./sources/*.wav; do
    fmt=$(ffprobe -v error -select_streams a:0 \
        -show_entries stream=sample_fmt \
        -of default=noprint_wrappers=1:nokey=1 "$f")

    if [ "$fmt" != "s16" ]; then
        echo "$f : $fmt"
    fi
done