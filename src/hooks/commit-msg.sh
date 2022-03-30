#!/usr/bin/env bash

commit_msg_path=$1
if !(test -f emojis.json); then
    echo "Fetching emoji list from GitHub..."
    curl -s \
        -H "Accept: application/vnd.github.v3+json" \
        https://api.github.com/emojis \
        -o emojis.json
fi
cat $commit_msg_path | grep -oP "(?<=:)(\S{1,})(?=:)" | while read -r code; do
    if !(grep -q '"$code"' emojis.json); then
        echo "Invalid emoji shortcode '$code'" 1>&2
        exit 1
    fi
done
