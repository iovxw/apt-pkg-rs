#include <apt-pkg/cachefile.h>
#include <apt-pkg/pkgcache.h>

#define rust_fn extern "C"

rust_fn bool init_config() {
  return pkgInitConfig(*_config);
}
rust_fn bool init_system() {
  return pkgInitSystem(*_config, _system);
}

rust_fn pkgCacheFile *pkg_cache_file_new() {
  return new pkgCacheFile();
}
rust_fn pkgCache *pkg_cache_get_pkg_cache(pkgCacheFile *self) {
  return self->GetPkgCache();
}
rust_fn pkgDepCache *pkg_cache_get_dep_cache(pkgCacheFile *self) {
  return self->GetDepCache();
}
rust_fn pkgPolicy *pkg_cache_get_policy(pkgCacheFile *self) {
  return self->GetPolicy();
}
rust_fn pkgSourceList *pkg_cache_get_source_list(pkgCacheFile *self) {
  return self->GetSourceList();
}
