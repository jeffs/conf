#!/bin/sh
viddy -n 1 "jj status --color=always | rg -v '^[WP]'; jj --color=always"
