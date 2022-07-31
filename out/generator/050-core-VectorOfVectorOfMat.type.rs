pub type VectorOfVectorOfMat = core::Vector<core::Vector<core::Mat>>;

impl VectorOfVectorOfMat {
	pub fn as_raw_VectorOfVectorOfMat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Mat>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfMat_new, cv_VectorOfVectorOfMat_delete,
	cv_VectorOfVectorOfMat_len, cv_VectorOfVectorOfMat_is_empty,
	cv_VectorOfVectorOfMat_capacity, cv_VectorOfVectorOfMat_resize,
	cv_VectorOfVectorOfMat_shrink_to_fit,
	cv_VectorOfVectorOfMat_reserve, cv_VectorOfVectorOfMat_remove,
	cv_VectorOfVectorOfMat_swap, cv_VectorOfVectorOfMat_clear,
	cv_VectorOfVectorOfMat_get, cv_VectorOfVectorOfMat_set,
	cv_VectorOfVectorOfMat_push, cv_VectorOfVectorOfMat_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Mat> }

unsafe impl Send for core::Vector<core::Vector<core::Mat>> {}

