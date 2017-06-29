/*  FAT12 Module
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

use super::RawFileSystem;


pub struct Fat12 {

}

impl RawFileSystem for Fat12 {
    fn create_file(path: &str) -> Option<()> {
        unimplemented!()
    }
    fn delete_file(path: &str) -> Option<()> {
        unimplemented!()
    }
}
