pub type VectorOfMat = core::Vector<core::Mat>;

impl VectorOfMat {
	pub fn as_raw_VectorOfMat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Mat, *const c_void, *mut c_void,
	cv_VectorOfMat_new, cv_VectorOfMat_delete,
	cv_VectorOfMat_len, cv_VectorOfMat_is_empty,
	cv_VectorOfMat_capacity, cv_VectorOfMat_resize,
	cv_VectorOfMat_shrink_to_fit,
	cv_VectorOfMat_reserve, cv_VectorOfMat_remove,
	cv_VectorOfMat_swap, cv_VectorOfMat_clear,
	cv_VectorOfMat_get, cv_VectorOfMat_set,
	cv_VectorOfMat_push, cv_VectorOfMat_insert,
}
vector_non_copy_or_bool! { clone core::Mat }

unsafe impl Send for core::Vector<core::Mat> {}

