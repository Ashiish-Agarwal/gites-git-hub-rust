use std::io::{BufRead, BufReader, Read};
use flate2:: read::ZlibDecoder ;

use crate::object::Object;


// 44c864df99eb8d8927375bef681661bf90dc92c6 
pub(crate) fn invoke(name_only:bool,tree_hash:&str)->anyhow::Result<()>{

    // two cammands we have 
    // 1: cargo run -- ls-tree --name-only (prints only names of file)
    // 2: cargo run -- ls-tree (print proper tree hash and other )
    
let kind = Object::read(&tree_hash);


 let   hash =  std::fs::File::open( format!(".git/objects/{}/{}",&tree_hash[..2],&tree_hash[2..]))?;
 let  decoder = ZlibDecoder::new(&hash);
 let mut  buffreader= BufReader::new( decoder);

 let mut buf= Vec::new();
 loop {
     buf.clear();
   let n =   buffreader.read_until(0,&mut buf)?;
   if n==0 {
    break;
       
   }
   let mut buffer = [0;20];

   let _tree = buffreader.read_exact(&mut buffer)?;
   let hexencode = hex::encode(buffer);
   let name_tree= String::from_utf8_lossy(&buf);

if name_only {
   println!("name_only {} ",name_tree.trim_end_matches('\0'));
    
}else {
     println!("trees {} {}",name_tree.trim_end_matches('\0'),hexencode);
}





    }


    Ok(())

}