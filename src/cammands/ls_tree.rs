use std::{fmt::format, fs, io::{BufRead, BufReader, Read}};

use clap::ValueHint::Unknown;
use flate2::bufread::{self, ZlibDecoder};



pub(crate) fn invoke(name_only:bool,tree_hash:&str)->anyhow::Result<()>{

 let   hash =  fs::read( format!(".git/objects/{}/{}",&tree_hash[..2],&tree_hash[2..]))?;
 let  decoder = ZlibDecoder::new(&hash[..]);
 let mut  buffreader= BufReader::new( decoder);

 let mut buf= Vec::new();
 loop {
     buf.clear();
   let n =   buffreader.read_until(0,&mut buf)?;
   if n==0 {
    break;
       
   }
   let mut buffer = [0;20];

   let tree = buffreader.read_exact(&mut buffer)?;
   let hexencode = hex::encode(buffer);

//    format!("gites file {:?}",tree);
   println!("tree files {:?}",hexencode);
   println!("tree files {:?}",buf);





    }


    Ok(())

}