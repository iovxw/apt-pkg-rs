extern crate libc;
extern crate apt_pkg_sys as ffi;

pub struct Cache {
    inner: *mut ffi::PkgCache,
}
impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            ffi::pkg_cache_delete(self.inner);
        }
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
        Cache { inner: inner }
    }
}

pub fn init() -> bool {
    unsafe { ffi::init_config() && ffi::init_system() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(true, init());
    }
}
