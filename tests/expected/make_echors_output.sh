#!/usr/bin/env zsh

OUTDIR="../expected/echo-rs"
mkdir -p $OUTDIR

echo "Hello" > $OUTDIR/hello.txt
echo -n "Hello" > $OUTDIR/hello-n.txt
echo "Hello there" > $OUTDIR/hello-there.txt
echo -n "Hello there" > $OUTDIR/hello-there-n.txt
