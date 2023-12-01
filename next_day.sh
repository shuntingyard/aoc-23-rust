#!/bin/sh
DAY=$(TZ=":US/Eastern" date +"%d")
DIR="day-$DAY"

[ -d "$DIR" ] && >&2 echo "$DIR exists, no action..." && exit 0

just create "$DIR"
