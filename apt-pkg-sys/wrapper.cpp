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
rust_fn void pkg_cache_file_delete(pkgCacheFile *self) {
  delete self;
}
rust_fn pkgCache *pkg_cache_file_get_pkg_cache(pkgCacheFile *self) {
  return self->GetPkgCache();
}
rust_fn pkgDepCache *pkg_cache_file_get_dep_cache(pkgCacheFile *self) {
  return self->GetDepCache();
}
rust_fn pkgPolicy *pkg_cache_file_get_policy(pkgCacheFile *self) {
  return self->GetPolicy();
}
rust_fn pkgSourceList *pkg_cache_file_get_source_list(pkgCacheFile *self) {
  return self->GetSourceList();
}

rust_fn void pkg_cache_delete(pkgCache *self) {
  delete self;
}
rust_fn pkgCache::PkgIterator pkg_cache_pkg_begin(pkgCache *self) {
  return self->PkgBegin();
}

rust_fn void pkg_cache_pkg_iterator_delete(pkgCache::PkgIterator *self) {
  delete self;
}
rust_fn pkgCache::PkgIterator pkg_cache_pkg_iterator_plusplus(pkgCache::PkgIterator *self) {
  return *self++;
}
rust_fn const char *pkg_cache_pkg_iterator_name(pkgCache::PkgIterator *self) {
  return self->Name();
}
rust_fn bool pkg_cache_pkg_iterator_end(pkgCache::PkgIterator *self) {
  return self->end();
}
