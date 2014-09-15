use std::io::{BufferedReader, File, stdin};
use std::num::FromPrimitive;
use std::os;

static BUFFER_SIZE: uint = 100;

#[allow(non_camel_case_types)]
#[deriving(PartialEq, FromPrimitive, Show)]
enum Foptypes{
    GF_FOP_MKNOD=3,
    GF_FOP_MKDIR=4,
    GF_FOP_UNLINK=5,
    GF_FOP_RMDIR=6,
    GF_FOP_SYMLINK=7,
    GF_FOP_RENAME=8,
    GF_FOP_LINK=9,
    GF_FOP_SETXATTR=17,
    GF_FOP_REMOVEXATTR=19,
    GF_FOP_CREATE=23,
    GF_FOP_SETATTR=38,
}

impl Foptypes{
    fn get(&self) -> &str{
        match *self{
            GF_FOP_MKNOD => "MKNOD",
            GF_FOP_MKDIR => "MKDIR",
            GF_FOP_UNLINK => "UNLINK",
            GF_FOP_RMDIR => "RMDIR",
            GF_FOP_SYMLINK => "SYMLINK",
            GF_FOP_RENAME => "RENAME",
            GF_FOP_LINK => "LINK",
            GF_FOP_SETXATTR => "SETXATTR",
            GF_FOP_REMOVEXATTR => "REMOVEXATTR",
            GF_FOP_CREATE => "CREATE",
            GF_FOP_SETATTR => "SETATTR"
        }
    }

    fn num_fields(&self) -> uint{
        match *self{
            GF_FOP_MKNOD => 7,
            GF_FOP_MKDIR => 7,
            GF_FOP_UNLINK => 4,
            GF_FOP_RMDIR => 4,
            GF_FOP_SYMLINK => 4,
            GF_FOP_RENAME => 5,
            GF_FOP_LINK => 4,
            GF_FOP_SETXATTR => 3,
            GF_FOP_REMOVEXATTR => 3,
            GF_FOP_CREATE => 7,
            GF_FOP_SETATTR => 3
        }
    }
}

#[deriving(Show)]
struct Parser{
    record_complete: bool,
    fop_type: char,
    token: String,
    tokens: Vec<String>,
    sep: char
}


impl Parser{
    fn new() -> Parser{
        Parser{record_complete: true,
               fop_type: '.',
               token: "".to_string(),
               tokens: Vec::new(),
               sep: '\x00'}
    }

    fn get_num_tokens(&self) -> uint {
        match self.fop_type{
            'E' => 7,
            'M' => 3,
            'D' => 2,
            _ => 0
        }
    }

    fn get_fop_enum(&self, v: String) -> Option<Foptypes>{
        let key:int = from_str::<int>(v.as_slice()).unwrap();
        let ty: Option<Foptypes> = FromPrimitive::from_int(key);
        return ty;
    }

    fn process_record(&self){
        let mut op: String = String::new();
        let mut i: int = 0;
        for v in self.tokens.iter(){
            if (self.fop_type == 'E' || self.fop_type == 'M') && i == 2 {
                let ty: Option<Foptypes> = self.get_fop_enum(v.clone());
                match ty{
                    Some(aa) => {
                        op.push_str(aa.get());
                        op.push_str(" ");
                    },
                    None => {}
                }
            }
            else{
                op.push_str(v.as_slice());
                op.push_str(" ");
            }
            i += 1;
        }
        println!("{}", op.as_slice().trim_chars(' '));
    }

    fn if_record_complete(&mut self){
        if self.token.as_slice() != ""{
            self.tokens.push(self.token.clone());
            self.token = "".to_string();
        }

        let mut num_tokens: uint = self.get_num_tokens();
        if (self.fop_type == 'E' || self.fop_type == 'M') && self.tokens.len() >= 3{
            let ty: Option<Foptypes> = self.get_fop_enum(self.tokens[2].clone());
            num_tokens = match ty{
                Some(aa) => aa.num_fields(),
                None => 0
            }
        }
        
        if num_tokens == self.tokens.len(){
            self.record_complete = true;
            self.process_record();
            self.tokens = Vec::new();
        }
    }

    fn parse_chunk(&mut self, chunk: &str){
        for (i, c) in chunk.chars().enumerate(){
            if self.record_complete && (c == 'E' || c == 'M' || c == 'D'){
                self.fop_type = c;
                self.tokens.push(String::from_char(1, c));
                self.record_complete = false;
                continue;
            }

            if c == self.sep{
                self.if_record_complete();
            }
            else{
                self.token.push_str(String::from_char(1, c).as_slice());
            }
        }
    }

    fn parse<T:Reader>(&mut self, mut reader: BufferedReader<T>){
        let buf:&mut [u8] = [0, ..BUFFER_SIZE];
        // read and ignore header
        let _header = reader.read_line();
        
        loop{
            match reader.read(buf) {
                Ok(nread) => self.parse_chunk(std::str::from_utf8(buf.slice(0, nread)).unwrap()),
                Err(_e) => {
                    break;
                }
            }
        }
        // process last record
        self.if_record_complete();
    }
}

fn main() {
    let mut parser = Parser::new();
    let reader = stdin();
    if reader.get_ref().isatty(){
        let args = os::args();
        let file = File::open(&Path::new(args[1].clone()));
        let reader_file = BufferedReader::new(file);
        parser.parse(reader_file);
    }
    else{
        parser.parse(reader);
    }
}
