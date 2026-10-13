#!/bin/sh
#
# TODO: Support showing only files Added or Modified in jj status.
viddy --interval 1s --exec tree --color=always "$@"
