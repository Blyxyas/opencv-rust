pub type VectorOfGMat = core::Vector<crate::gapi::GMat>;

impl VectorOfGMat {
	pub fn as_raw_VectorOfGMat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GMat, *const c_void, *mut c_void,
	cv_VectorOfGMat_new, cv_VectorOfGMat_delete,
	cv_VectorOfGMat_len, cv_VectorOfGMat_is_empty,
	cv_VectorOfGMat_capacity, cv_VectorOfGMat_resize,
	cv_VectorOfGMat_shrink_to_fit,
	cv_VectorOfGMat_reserve, cv_VectorOfGMat_remove,
	cv_VectorOfGMat_swap, cv_VectorOfGMat_clear,
	cv_VectorOfGMat_get, cv_VectorOfGMat_set,
	cv_VectorOfGMat_push, cv_VectorOfGMat_insert,
}
vector_non_copy_or_bool! { crate::gapi::GMat }

unsafe impl Send for core::Vector<crate::gapi::GMat> {}

