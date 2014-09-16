gchangelogparser
================

GlusterFS Changelogs parser in [Rust](http://rust-lang.org/).

## Why?

Converts this

    GlusterFS Changelog | version: v1.1 | encoding : 2
    E0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0^@4^@16877^@0^@0^@00000000-0000-0000-0000-000000000001/dir1^@Ec5250af6-720e-4bfe-b938-827614304f39^@23^@33188^@0^@0^@0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0/hello.txt^@Dc5250af6-720e-4bfe-b938-827614304f39^@Dc5250af6-720e-4bfe-b938-827614304f39^@

to human readable :)

    E 0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0 MKDIR 16877 0 0 00000000-0000-0000-0000-000000000001/dir1
    E c5250af6-720e-4bfe-b938-827614304f39 CREATE 33188 0 0 0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0/hello.txt
    D c5250af6-720e-4bfe-b938-827614304f39
    D c5250af6-720e-4bfe-b938-827614304f39

## Installation

Download the binary file from [here](https://github.com/aravindavk/gchangelogparser/releases/tag/v0.0.1) and copy to `/usr/local/bin` or any other directory which is available in PATH.

Or you can clone the github repo and compile it in your system.

    git clone https://github.com/aravindavk/gchangelogparser.git
    cd gchangelogparser
    cargo build

Then copy the binary `gchangelogparser` created in target to /usr/local/bin(or any other dir which is available in env var PATH)

## How to use?

For example, changelog related to brick1 are available in `/bricks/brick1/.glusterfs/changelogs/`, to use gchangelogparser,

    gchangelogparser /bricks/brick1/.glusterfs/changelogs/CHANGELOG.1410542281

or

    cat /bricks/brick1/.glusterfs/changelogs/CHANGELOG.1410542281 | gchangelogparser

To parse and display changelog content for all the changelogs in brick, create a script called `all_changes.sh`

    #!/bin/bash
    # filename: all_changes.sh
    changelogs=$(find $1 -type f -name "CHANGELOG.*" -size +51c);
    for cf in $changelogs
    do
        cat $cf | gchangelogparser
    done;

To run

    ./all_changes.sh /bricks/brick1/.glusterfs/changelogs/

To see the events happened for particular GFID(GlusterFS ID for each file/dir),

    ./all_changes.sh /bricks/brick1/.glusterfs/changelogs/ | grep " 7db2b971-7516-40bb-b069-90c875960b0a "
