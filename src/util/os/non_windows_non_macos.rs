use libc::{O_RDONLY, c_ulong, close, ioctl, open} ;
use std::os::unix::fs::FileTypeExt ;
use std::os::unix::ffi::OsStrExt ;
use std::fs::{FileType, Metadata} ;
use std::ffi::CString ;
use std::path::Path ;

include!(concat!(env!("OUT_DIR"), "/ioctl-data/ioctrl.rs")) ;

/// OS-specific check for fileness
/// FileTypeで判別できるのは、ディレクトリ(is_dir)、ただのファイル(is_file)、シンボリックリンク(is_symlink)、
pub fn is_device(tp: &FileType) -> bool {
    tp.is_block_device() || tp.is_char_device() || tp.is_fifo() || tp.is_socket()    
}

// 全く関係ないけど、今日はプリミティブという単語をど忘れして、話せなくなった。