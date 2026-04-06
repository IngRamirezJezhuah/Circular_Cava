#!/bin/bash

DEFAULT_COVER="$(pwd)/../eww/default.png"
COVER="/tmp/music-dot-cover.png"

get_data() {
  status=$(playerctl status 2>/dev/null || echo "Offline")
  title=$(playerctl metadata title 2>/dev/null || echo "Nada Sonando...")
  artist=$(playerctl metadata artist 2>/dev/null || echo "...")
  art_url=$(playerctl metadata mpris:artUrl 2>/dev/null)

  pos=$(playerctl position 2>/dev/null | cut -d'.' -f1)
  len_us=$(playerctl metadata mpris:length 2>/dev/null)
  
  [[ -z "$pos" ]] && pos=0
  if [[ -z "$len_us" ]]; then len=100; else len=$((len_us / 1000000)); fi
  
  if [[ -z  "$art_url" ]]; then
      cp "$DEFAULT_COVER" "$COVER" 2>/dev/null || touch "$COVER"
    elif [[ "$art_url" == http* ]]; then
      curl -s "$art_url" -o "$COVER"
    elif [[ "$art_url" == file://* ]]; then
      cp "${art_url#file://}" "$COVER"
  fi
  
  jq --unbuffered -c -n --arg title "$title" --arg artist "$artist" --arg status "$status" --arg cover "$COVER" --arg pos "$pos" --arg len "$len" '{title: $title, artist: $artist, status: $status, cover: $cover, position: $pos, length: $len}'
}

while true; do
  get_data
  sleep 0.5
done
