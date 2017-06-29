/*  Architecture Dependent Filesystem Module
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

pub mod fat12;
pub mod vfs;

pub fn init_fs(){
    
}

pub trait RawFileSystem {
    fn create_file(path: &str) -> Option<()>;
    fn delete_file(path: &str) -> Option<()>;
    // TODO: Interfaces for reading and writing files
}