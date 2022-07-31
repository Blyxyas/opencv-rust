pub type VectorOfGRunArg = core::Vector<crate::gapi::GRunArg>;

impl VectorOfGRunArg {
	pub fn as_raw_VectorOfGRunArg(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGRunArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GRunArg, *const c_void, *mut c_void,
	cv_VectorOfGRunArg_new, cv_VectorOfGRunArg_delete,
	cv_VectorOfGRunArg_len, cv_VectorOfGRunArg_is_empty,
	cv_VectorOfGRunArg_capacity, cv_VectorOfGRunArg_resize,
	cv_VectorOfGRunArg_shrink_to_fit,
	cv_VectorOfGRunArg_reserve, cv_VectorOfGRunArg_remove,
	cv_VectorOfGRunArg_swap, cv_VectorOfGRunArg_clear,
	cv_VectorOfGRunArg_get, cv_VectorOfGRunArg_set,
	cv_VectorOfGRunArg_push, cv_VectorOfGRunArg_insert,
}
vector_non_copy_or_bool! { crate::gapi::GRunArg }

unsafe impl Send for core::Vector<crate::gapi::GRunArg> {}

