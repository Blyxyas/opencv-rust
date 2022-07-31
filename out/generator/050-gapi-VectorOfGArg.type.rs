pub type VectorOfGArg = core::Vector<crate::gapi::GArg>;

impl VectorOfGArg {
	pub fn as_raw_VectorOfGArg(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GArg, *const c_void, *mut c_void,
	cv_VectorOfGArg_new, cv_VectorOfGArg_delete,
	cv_VectorOfGArg_len, cv_VectorOfGArg_is_empty,
	cv_VectorOfGArg_capacity, cv_VectorOfGArg_resize,
	cv_VectorOfGArg_shrink_to_fit,
	cv_VectorOfGArg_reserve, cv_VectorOfGArg_remove,
	cv_VectorOfGArg_swap, cv_VectorOfGArg_clear,
	cv_VectorOfGArg_get, cv_VectorOfGArg_set,
	cv_VectorOfGArg_push, cv_VectorOfGArg_insert,
}
vector_non_copy_or_bool! { crate::gapi::GArg }

unsafe impl Send for core::Vector<crate::gapi::GArg> {}

