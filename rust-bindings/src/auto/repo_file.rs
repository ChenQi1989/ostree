// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Repo;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "OstreeRepoFile")]
    pub struct RepoFile(Object<ffi::OstreeRepoFile, ffi::OstreeRepoFileClass>) @implements gio::File;

    match fn {
        type_ => || ffi::ostree_repo_file_get_type(),
    }
}

impl RepoFile {
    #[doc(alias = "ostree_repo_file_ensure_resolved")]
    pub fn ensure_resolved(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_repo_file_ensure_resolved(self.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_repo_file_get_checksum")]
    #[doc(alias = "get_checksum")]
    pub fn checksum(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ostree_repo_file_get_checksum(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_get_repo")]
    #[doc(alias = "get_repo")]
    pub fn repo(&self) -> Repo {
        unsafe {
            from_glib_none(ffi::ostree_repo_file_get_repo(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_get_root")]
    #[doc(alias = "get_root")]
#[must_use]
    pub fn root(&self) -> RepoFile {
        unsafe {
            from_glib_none(ffi::ostree_repo_file_get_root(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_get_xattrs")]
    #[doc(alias = "get_xattrs")]
    pub fn xattrs(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut out_xattrs = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_repo_file_get_xattrs(self.to_glib_none().0, &mut out_xattrs, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_xattrs)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_repo_file_tree_find_child")]
    pub fn tree_find_child(&self, name: &str) -> (i32, bool, glib::Variant) {
        unsafe {
            let mut is_dir = mem::MaybeUninit::uninit();
            let mut out_container = ptr::null_mut();
            let ret = ffi::ostree_repo_file_tree_find_child(self.to_glib_none().0, name.to_glib_none().0, is_dir.as_mut_ptr(), &mut out_container);
            (ret, from_glib(is_dir.assume_init()), from_glib_full(out_container))
        }
    }

    #[doc(alias = "ostree_repo_file_tree_get_contents")]
    pub fn tree_get_contents(&self) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::ostree_repo_file_tree_get_contents(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_tree_get_contents_checksum")]
    pub fn tree_get_contents_checksum(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ostree_repo_file_tree_get_contents_checksum(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_tree_get_metadata")]
    pub fn tree_get_metadata(&self) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::ostree_repo_file_tree_get_metadata(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_tree_get_metadata_checksum")]
    pub fn tree_get_metadata_checksum(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ostree_repo_file_tree_get_metadata_checksum(self.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_repo_file_tree_query_child")]
    pub fn tree_query_child(&self, n: i32, attributes: &str, flags: gio::FileQueryInfoFlags, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<gio::FileInfo, glib::Error> {
        unsafe {
            let mut out_info = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_repo_file_tree_query_child(self.to_glib_none().0, n, attributes.to_glib_none().0, flags.into_glib(), &mut out_info, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_info)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_repo_file_tree_set_metadata")]
    pub fn tree_set_metadata(&self, checksum: &str, metadata: &glib::Variant) {
        unsafe {
            ffi::ostree_repo_file_tree_set_metadata(self.to_glib_none().0, checksum.to_glib_none().0, metadata.to_glib_none().0);
        }
    }
}

impl fmt::Display for RepoFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepoFile")
    }
}
