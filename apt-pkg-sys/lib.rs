extern crate libc;

use libc::{c_int, c_uint, c_void};

pub enum PkgCacheFile {}
pub enum PkgCache {}
pub enum PkgDepCache {}
pub enum PkgPolicy {}
pub enum PkgSourceList {}

extern "C" {
    pub fn init_config() -> bool;
    pub fn init_system() -> bool;
    pub fn pkg_cache_file_new() -> *mut PkgCacheFile;
    pub fn pkg_cache_file_get_pkg_cache(var: *mut PkgCacheFile) -> *mut PkgCache;
    pub fn pkg_cache_file_get_dep_cache(var: *mut PkgCacheFile) -> *mut PkgDepCache;
    pub fn pkg_cache_file_get_policy(var: *mut PkgCacheFile) -> *mut PkgPolicy;
    pub fn pkg_cache_file_get_source_list(var: *mut PkgCacheFile) -> *mut PkgSourceList;
}
