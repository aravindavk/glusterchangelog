use std::env;
extern crate glusterchangelog;

use glusterchangelog::Record;

fn parse_record(record: &Record){
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

fn main(){
    let filename = env::args().nth(1).unwrap();
    match glusterchangelog::parse(filename, parse_record){
        Err(e) => println!("Error: {}", e),
        _ => {}
    }
}
