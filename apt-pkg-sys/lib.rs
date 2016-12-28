extern crate libc;

use libc::{c_long, c_void, c_char};

pub enum PkgCacheFile {}
pub enum PkgCache {}
pub enum PkgDepCache {}
pub enum PkgPolicy {}
pub enum PkgSourceList {}

#[repr(C)]
pub struct PkgIterator {
    pkg: *mut c_void,
    owner: *mut PkgCache,
    hash_index: c_long,
}

extern "C" {
    pub fn init_config() -> bool;
    pub fn init_system() -> bool;

    pub fn pkg_cache_file_new() -> *mut PkgCacheFile;
    pub fn pkg_cache_file_delete(var: *mut PkgCacheFile) -> c_void;
    pub fn pkg_cache_file_get_pkg_cache(var: *mut PkgCacheFile) -> *mut PkgCache;
    pub fn pkg_cache_file_get_dep_cache(var: *mut PkgCacheFile) -> *mut PkgDepCache;
    pub fn pkg_cache_file_get_policy(var: *mut PkgCacheFile) -> *mut PkgPolicy;
    pub fn pkg_cache_file_get_source_list(var: *mut PkgCacheFile) -> *mut PkgSourceList;

    pub fn pkg_cache_delete(var: *mut PkgCache) -> c_void;
    pub fn pkg_cache_pkg_begin(var: *mut PkgCache) -> PkgIterator;

    pub fn pkg_cache_pkg_iterator_delete(var: *mut PkgIterator) -> c_void;
    pub fn pkg_cache_pkg_iterator_plusplus(var: *mut PkgIterator) -> PkgIterator;
    pub fn pkg_cache_pkg_iterator_name(var: *mut PkgIterator) -> *const c_char;
    pub fn pkg_cache_pkg_iterator_end(var: *mut PkgIterator) -> bool;
}
