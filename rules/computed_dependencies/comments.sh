#!/bin/bash
grep -v '^ *#' $1 > $2  # Remove lines with only a Python-style comment
