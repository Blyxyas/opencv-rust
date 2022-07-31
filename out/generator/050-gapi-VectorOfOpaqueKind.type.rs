pub type VectorOfOpaqueKind = core::Vector<crate::gapi::OpaqueKind>;

impl VectorOfOpaqueKind {
	pub fn as_raw_VectorOfOpaqueKind(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfOpaqueKind(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::gapi::OpaqueKind, *const c_void, *mut c_void,
	cv_VectorOfOpaqueKind_new, cv_VectorOfOpaqueKind_delete,
	cv_VectorOfOpaqueKind_len, cv_VectorOfOpaqueKind_is_empty,
	cv_VectorOfOpaqueKind_capacity, cv_VectorOfOpaqueKind_resize,
	cv_VectorOfOpaqueKind_shrink_to_fit,
	cv_VectorOfOpaqueKind_reserve, cv_VectorOfOpaqueKind_remove,
	cv_VectorOfOpaqueKind_swap, cv_VectorOfOpaqueKind_clear,
	cv_VectorOfOpaqueKind_get, cv_VectorOfOpaqueKind_set,
	cv_VectorOfOpaqueKind_push, cv_VectorOfOpaqueKind_insert,
}
vector_copy_non_bool! { crate::gapi::OpaqueKind, *const c_void, *mut c_void,
	cv_VectorOfOpaqueKind_data, cv_VectorOfOpaqueKind_data_mut, cv_VectorOfOpaqueKind_from_slice,
	cv_VectorOfOpaqueKind_clone,
}

unsafe impl Send for core::Vector<crate::gapi::OpaqueKind> {}

