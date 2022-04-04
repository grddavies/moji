#!/usr/bin/env sh
commit_msg_path=$1; MOJI_HOME="$HOME/.moji"; EMOJI_DICT="$MOJI_HOME/emojis.json";
if !(test -f $EMOJI_DICT); then echo "Fetching emoji list from GitHub..." curl -s -H "Accept: application/vnd.github.v3+json" https://api.github.com/emojis --create-dirs -o $EMOJI_DICT; fi; cat $commit_msg_path | grep -oP "(?<=:)(\S{1,})(?=:)" | while read -r code; do if !(grep -q \"$code\" $EMOJI_DICT); then echo "Invalid emoji shortcode '$code'" 1>&2; exit 1; fi
done