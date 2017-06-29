/*  Architecture Dependent Filesystem Module
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

pub mod fat12;
pub mod vfs;

pub fn init_fs(){
    
}

pub struct File {

}

pub trait RawFileSystem {
    fn create(path: &str) -> Option<()>;
    fn delete(path: &str) -> Option<()>;
    /*
    fn open(path: &str) -> File;
    fn close(file: File);
    fn read(file: File) -> &[u8];
    fn write(file: File, &[u8]);
    */
}