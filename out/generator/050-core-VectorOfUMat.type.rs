pub type VectorOfUMat = core::Vector<core::UMat>;

impl VectorOfUMat {
	pub fn as_raw_VectorOfUMat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfUMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::UMat, *const c_void, *mut c_void,
	cv_VectorOfUMat_new, cv_VectorOfUMat_delete,
	cv_VectorOfUMat_len, cv_VectorOfUMat_is_empty,
	cv_VectorOfUMat_capacity, cv_VectorOfUMat_resize,
	cv_VectorOfUMat_shrink_to_fit,
	cv_VectorOfUMat_reserve, cv_VectorOfUMat_remove,
	cv_VectorOfUMat_swap, cv_VectorOfUMat_clear,
	cv_VectorOfUMat_get, cv_VectorOfUMat_set,
	cv_VectorOfUMat_push, cv_VectorOfUMat_insert,
}
vector_non_copy_or_bool! { clone core::UMat }

unsafe impl Send for core::Vector<core::UMat> {}

