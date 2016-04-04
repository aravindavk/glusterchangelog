glusterchangelog
================

GlusterFS Changelogs parser written using [Rust](http://rust-lang.org/).

## Why?

Converts this format

    GlusterFS Changelog | version: v1.1 | encoding : 2
    E0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0^@4^@16877^@0^@0^@00000000-00
    00-0000-0000-000000000001/dir1^@Ec5250af6-720e-4bfe-b938-827614304
    f39^@23^@33188^@0^@0^@0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0/hello.t
    xt^@Dc5250af6-720e-4bfe-b938-827614304f39^@Dc5250af6-720e-4bfe-b93
    8-827614304f39^@

to human readable format :)

    E 0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0 MKDIR 16877 0 0 \
        00000000-0000-0000-0000-000000000001/dir1
    E c5250af6-720e-4bfe-b938-827614304f39 CREATE 33188 0 0 \
        0b99ef11-4b79-4cd0-9730-b5a0e8c4a8c0/hello.txt
    D c5250af6-720e-4bfe-b938-827614304f39
    D c5250af6-720e-4bfe-b938-827614304f39

## Installation

Clone the github repo and compile it in your system.

    git clone https://github.com/aravindavk/glusterchangelog.git
    cd glusterchangelog
    cargo build --release

Then copy the binary `glusterchangelogparser` created in `target/release`
directory to `/usr/local/bin`(or any other dir which is available in env
var PATH)

## Using it as a Tool

For example, changelog related to brick1 are available in
`/bricks/brick1/.glusterfs/changelogs/`, to use glusterchangelogparser,

    glusterchangelogparser /bricks/brick1/.glusterfs/changelogs/CHANGELOG.1410542281

To parse and display changelog content for all the changelogs in
brick, create a script called `all_changes.sh`

    #!/bin/bash
    # filename: all_changes.sh
    BRICK_ROOT=$1;
    for cf in `ls ${BRICK_ROOT}/.glusterfs/changelogs/CHANGELOG.*`
    do
        glusterchangelogparser $cf
    done;

To run

    ./all_changes.sh /bricks/brick1

To see the events happened for particular GFID(GlusterFS ID for each file/dir),

    ./all_changes.sh /bricks/brick1 | grep "7db2b971-7516-40bb-b069-90c875960b0a"

## Using as Library

First, add the following to your `Cargo.toml`

    [dependencies]
    glusterchangelog = "0.1.2"

Add to your code,

    extern crate glusterchangelog;
    use glusterchangelog::Record;

Create a callback function, which accepts `glusterchangelog::Record` as input

    fn parse_record(record: &Record) {
        match record.fop_type{
            "D" => println!("{} D {}", record.ts, record.gfid),
            "M" => println!("{} M {} {}", record.ts, record.gfid, record.fop),
            "E" => {
                match record.fop{
                    "MKNOD" | "MKDIR" | "CREATE" => println!("{} E {} {} {} {} {} {}",
                                                             record.ts,
                                                             record.gfid,
                                                             record.fop,
                                                             record.path,
                                                             record.mode,
                                                             record.uid,
                                                             record.gid),
                    "LINK" | "SYMLINK" => println!("{} E {} {} {}",
                                                   record.ts,
                                                   record.gfid,
                                                   record.fop,
                                                   record.path),
                    "UNLINK" | "RMDIR" => println!("{} E {} {} {} {}",
                                                   record.ts,
                                                   record.gfid,
                                                   record.fop,
                                                   record.path,
                                                   record.fullpath),
                    "RENAME" => println!("{} E {} {} {} {}",
                                         record.ts,
                                         record.gfid,
                                         record.fop,
                                         record.path1,
                                         record.path2),
                    _ => {}
                }
            },
            _ => {}
        }
    }

Call the Parser,

    fn main(){
        let filename = env::args().nth(1).unwrap();
        match glusterchangelog::parse(filename, parse_record){
            Err(e) => println!("Error: {}", e),
            _ => {}
        }
    }


## License

The MIT License (MIT). See LICENSE file for more details.
