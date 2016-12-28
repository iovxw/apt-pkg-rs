extern crate libc;
extern crate apt_pkg_sys as ffi;

use libc::c_char;
use std::ffi::CStr;
use std::str;

pub struct Package {
    name: String,
}

pub struct Cache {
    inner: *mut ffi::PkgCache,
    iter: ffi::PkgIterator,
}
impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            ffi::pkg_cache_delete(self.inner);
        }
    }
}
impl Iterator for Cache {
    type Item = Package;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item>;
        if !unsafe { ffi::pkg_cache_pkg_iterator_end(&mut self.iter) } {
            result = Some(Package {
                name: c_char_ptr_to_string(unsafe {
                    ffi::pkg_cache_pkg_iterator_name(&mut self.iter)
                }),
            });
        } else {
            result = None;
        }
        self.iter = unsafe { ffi::pkg_cache_pkg_iterator_plusplus(&mut self.iter) };
        result
    }
}

pub struct CacheFile {
    inner: *mut ffi::PkgCacheFile,
}
impl Drop for CacheFile {
    fn drop(&mut self) {
        unsafe {
            ffi::pkg_cache_file_delete(self.inner);
        }
    }
}
impl CacheFile {
    pub fn new() -> CacheFile {
        let inner = unsafe { ffi::pkg_cache_file_new() };
        CacheFile { inner: inner }
    }
    pub fn get_pkg_cache(&self) -> Cache {
        let inner = unsafe { ffi::pkg_cache_file_get_pkg_cache(self.inner) };
        let iter = unsafe { ffi::pkg_cache_pkg_begin(inner) };
        Cache {
            inner: inner,
            iter: iter,
        }
    }
}

pub fn init() -> bool {
    unsafe { ffi::init_config() && ffi::init_system() }
}

fn c_char_ptr_to_string(c_buf: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    println!("{}", str_slice.to_owned());
    str_slice.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(true, init());
        let cache = CacheFile::new().get_pkg_cache();
        for pkg in cache {
            println!{"{}", pkg.name};
        }
    }
}
