use std::fs::File;
use std::io;
use std::io::Read;

pub struct Record<'a> {
    pub ts: u64,
    pub fop_type: &'a str,
    pub fop: &'a str,
    pub gfid: &'a str,
    pub path: &'a str,
    pub path1: &'a str,
    pub path2: &'a str,
    pub mode: i64,
    pub uid: i64,
    pub gid: i64,
    pub fullpath: &'a str
}

#[no_mangle]
pub extern fn parse(filename: String,
                    callback: fn (record: &Record)) -> Result<(), io::Error>{
    let ts = filename.split(".")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap_or(0);

    match File::open(filename){
        Ok(mut file) => {
            let mut data = String::new();
            let _res = file.read_to_string(&mut data);
            let raw_data: Vec<&str> = data.lines().collect();
            if raw_data.len() == 1{
                // Empty Changelog
                return Ok(());
            }

            let records:Vec<&str> = raw_data[1]
                .split_terminator('\x00')
                .collect();
            let version = raw_data[0].split(" ").collect::<Vec<&str>>()[4];
            let mut i: usize = 0;
            while i < records.len(){
                i = parse_and_jump_to(callback, &records, i, version, ts);
            }
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}

fn type_num_to_str(fop: &str) -> &str{
    match fop {
        "3" => "MKNOD",
        "4" => "MKDIR",
        "5" => "UNLINK",
        "6" => "RMDIR",
        "7" => "SYMLINK",
        "8" => "RENAME",
        "9" => "LINK",
        "17" => "SETXATTR",
        "19" => "REMOVEXATTR",
        "23" => "CREATE",
        "38" => "SETATTR",
        "24" => "FTRUNCATE",
        _ => "NULL"
    }
}

fn new_create_mknod_mkdir_record<'a>(ts: u64,
                              fop: &'a str,
                              gfid: &'a str,
                              path: &'a str,
                              mode: i64,
                              uid: i64,
                              gid: i64) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: fop,
        gfid: gfid,
        path: path,
        path1: "",
        path2: "",
        mode: mode,
        uid: uid,
        gid: gid,
        fullpath: ""
    }
}

fn new_data_record<'a>(ts: u64, gfid: &'a str) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: "",
        gfid: gfid,
        path: "",
        path1: "",
        path2: "",
        mode: 0,
        uid: 0,
        gid: 0,
        fullpath: ""
    }
}

fn new_meta_record<'a>(ts: u64, gfid: &'a str, fop: &'a str) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: fop,
        gfid: gfid,
        path: "",
        path1: "",
        path2: "",
        mode: 0,
        uid: 0,
        gid: 0,
        fullpath: ""
    }
}

fn new_rename_record<'a>(ts: u64,
                         gfid: &'a str,
                         path1: &'a str,
                         path2: &'a str) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: "RENAME",
        gfid: gfid,
        path: "",
        path1: path1,
        path2: path2,
        mode: 0,
        uid: 0,
        gid: 0,
        fullpath: ""
    }
}

fn new_link_symlink_record<'a>(ts: u64,
                               fop: &'a str,
                               gfid: &'a str,
                               path: &'a str) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: fop,
        gfid: gfid,
        path: path,
        path1: "",
        path2: "",
        mode: 0,
        uid: 0,
        gid: 0,
        fullpath: ""
    }
}

fn new_unlink_rmdir_record<'a>(ts: u64,
                               fop: &'a str,
                               gfid: &'a str,
                               path: &'a str,
                               fullpath: &'a str) -> Record<'a>{
    Record{
        ts: ts,
        fop_type: "E",
        fop: fop,
        gfid: gfid,
        path: path,
        path1: "",
        path2: "",
        mode: 0,
        uid: 0,
        gid: 0,
        fullpath: fullpath
    }
}

fn parse_and_jump_to<'a>(callback: fn (record: &Record),
                         records: &Vec<&'a str>,
                         idx: usize,
                         version: &str,
                         ts: u64) -> usize {
    // If changelog.capture-del-path is off, then Changelog will not record
    // deleted path, so Zero byte is recorded in place of Path. Do not attempt
    // parsing if Batch start is Empty. Split (Type, GFID) will fail with error
    if records[idx] == ""{
        return idx + 1
    }

    match records[idx].split_at(1) {
        ("M", gfid) => {
            callback(&new_meta_record(
                ts,
                gfid,
                type_num_to_str(records[idx+1])));
            idx + 2
        },
        ("D", gfid) => {
            callback(&new_data_record(ts, gfid));
            idx + 1
        },
        ("E", gfid) => {
            let fop = type_num_to_str(records[idx + 1]);
            match fop{
                "MKNOD" | "MKDIR" | "CREATE" => {
                    callback(&new_create_mknod_mkdir_record(
                        ts,
                        fop,
                        gfid,
                        records[idx+5],
                        records[idx+2].parse::<i64>().unwrap_or(0),
                        records[idx+3].parse::<i64>().unwrap_or(0),
                        records[idx+4].parse::<i64>().unwrap_or(0)));
                    idx + 5
                },
                "LINK" | "SYMLINK" => {
                    callback(&new_link_symlink_record(
                        ts,
                        fop,
                        gfid,
                        records[idx+2]));
                    idx + 2
                },
                "RENAME" => {
                    callback(&new_rename_record(
                        ts,
                        gfid,
                        records[idx+2],
                        records[idx+3]));
                    idx + 3
                },
                "UNLINK" | "RMDIR" => {
                    match version{
                        "v1.1" => {
                            callback(&new_unlink_rmdir_record(
                                ts,
                                fop,
                                gfid,
                                records[idx+2],
                                ""));
                            idx + 2
                        },
                        _ => {
                            callback(&new_unlink_rmdir_record(
                                ts,
                                fop,
                                gfid,
                                records[idx+2],
                                records[idx+3]));
                            idx + 3
                        }
                    }
                },
                _ => idx + 1
            }
        }
        _ => idx + 1
    }
}
