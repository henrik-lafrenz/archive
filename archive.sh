#!/bin/bash

# jason parser required
if !command -v jq >/dev/null 2>&1 ; then
	echo "jq not found, please install"
	exit 0
fi

# print contents of the config file
config_file=./config.json
tmp_path="$( jq -r '.tmp_path' "$config_file" )"
echo "tmp_path: $tmp_path"
archive_path="$( jq -r '.archive_path' "$config_file" )"
echo "archive_path: $archive_path"

# first arg needs to be the command supposed to be executed
if [ "$#" -lt 1 ]; then
	echo "no command given"
	exit 0
fi

# open and rename need a match
if [ $1 = "open" ] || [ $1 = "rename" ]; then
	if [ "$#" -lt 2 ]; then
		echo "no match given"
		exit 0
	fi
fi

# search needs a search string
if [ $1 = "search" ]; then
	if [ "$#" -lt 2 ]; then
		echo "no search string given"
		exit 0
	fi
fi

# delegate to scripts
if [ $1 = "open" ]; then
	python3 open.py $archive_path $tmp_path $2
elif [ $1 = "investigate" ]; then
	python3 investigate.py $archive_path --match=$2 $3
elif [ $1 = "new" ]; then
	python3 new.py $tmp_path
elif [ $1 = "rename" ]; then
	python3 rename.py $archive_path $tmp_path $2 $3
elif [ $1 = "search" ]; then
	arch_path=$archive_path/ARCHIVE
	cd ./rust/search && cargo run $arch_path $2
else
	echo "not a valid command: $1"
	exit 0
fi
