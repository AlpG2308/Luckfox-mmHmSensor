#!/usr/bin/env bash

if [ $# -eq 0 ]; then
	echo 'No argument supplied'
else
	file=$1
	binwalk ${file}
fi
