use anyhow::{Context, Ok, Result, ensure};
use flate2::{ Compression, read::ZlibDecoder  ,write:: ZlibEncoder};
use clap::{Parser, Subcommand, builder::Str};
use std::{fmt::format, fs, io::{self, BufRead, BufReader, Read, Stdout, Write}, path::PathBuf, vec};
use std::ffi::CStr;
use sha1::{Digest, Sha1, digest};

#[derive(Parser)]
// #[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    Init ,
    Catfile {
        #[clap(short='p')]
        prettyprint:bool,
        objecthash:String
    },
    HashFile {
        #[clap(short='w')]
        write :bool,
        file:PathBuf


    }
}
enum Kind {
    blob
}

fn main()-> Result<()>{
    let cli = Cli::parse();
    
   
match cli.command {
   Commands::Init =>{

   fs::create_dir(".gites").unwrap();
   fs::create_dir(".gites/objects").unwrap();
   fs::create_dir(".gites/refs").unwrap();
   fs::write(".gites/HEAD", "ref:refs/heads/main\n").unwrap()
   


}
Commands::Catfile { prettyprint , objecthash}=>{
    anyhow::ensure!(prettyprint,"mode must be wihtout -p , and we dont support mode");
    let  f = std::fs::File::open(format!("./gites/objects/{}/{}", &objecthash[..2], &objecthash[2..])).context("open in .gites")?;

    let mut z = ZlibDecoder::new(f);
    let mut z = BufReader::new(z);
    let mut buff= Vec::new();
    z.read_until(0,&mut buff).expect("read header from gites");
    let header = CStr::from_bytes_until_nul(&buff).expect("know this is exactly null ");
    let header = header.to_str().expect("git object file is not valid");
    let Some((Kind, size))= header.split_once(" ")else {
        anyhow::bail!("git object file header is not known type: ' {header}'  ")
    };
    
   
    let kind = match Kind {
        "blob"=>Kind::blob,
        _=>anyhow::bail!("kwe do not yet know how to print a '{Kind}'")
};


    let size = size.parse::<usize>().context("get header file is invalid size ")?;

    
    buff.clear();
     buff.resize(size,0);
    z.read_exact(&mut buff[..]).context("read true content of .gites/object file")?;
    let n = z.read(&mut [0]).context("validate eof in .git/object file had {n} trailing bytes")?;
    let mut  z = Ratelimitor{
        reader:z,
        limit:size,
    };
   
    match kind {
       Kind::blob =>{
           let stdout = std::io::stdout();
           let mut stdout = stdout.lock();
           let n = std::io::copy(&mut z,&mut stdout).context("write .git/object file to stdout")?;
       }
       
   }
   
    let stdout = std::io::stdout();
    let mut Stdout = stdout.lock();



    


    
}
   
    
Commands::HashFile { write , file  }=>{
    
   
     let read = fs::read_to_string(file).unwrap();

    
let blobcontent= format!("blob{}\0{}",&read.len(), &read);

let hex_string=  hex::encode( Sha1::digest(&blobcontent));
let dir = format!(".gites/objects/{}",&hex_string[..2]);
let path = format!("{}/{}", &dir,&hex_string[2..]  );

let mut zlib_en= ZlibEncoder::new(Vec::new(),Compression::default());
zlib_en.write_all(blobcontent.as_bytes()).unwrap();

let compressed = zlib_en.finish().unwrap();
fs::create_dir_all(&dir);

fs::write(&path, compressed).unwrap();
println!("hex stirng {}", hex_string);



}
}
   
Ok(())

}

struct Ratelimitor <R>{

    reader:R,
    limit:usize


}
 
impl<R> Read for  Ratelimitor<R> where R:Read  {
    fn read(&mut self,mut  buf: &mut [u8]) -> std::io::Result<usize> {
       if buf.len()>self.limit{
        buf = &mut buf[..self.limit +1]
       }
       let n = self.reader.read(buf)?;
       if n > self.limit {
        return Err(io::Error::new(io::ErrorKind::Other,"too many bites"));
           
       }
       self.limit-=n;
       Ok(n);
        return Err(io::Error::new(io::ErrorKind::Other,"too many bites"));

    }
    

    
}