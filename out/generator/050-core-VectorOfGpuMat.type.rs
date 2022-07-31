pub type VectorOfGpuMat = core::Vector<core::GpuMat>;

impl VectorOfGpuMat {
	pub fn as_raw_VectorOfGpuMat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::GpuMat, *const c_void, *mut c_void,
	cv_VectorOfGpuMat_new, cv_VectorOfGpuMat_delete,
	cv_VectorOfGpuMat_len, cv_VectorOfGpuMat_is_empty,
	cv_VectorOfGpuMat_capacity, cv_VectorOfGpuMat_resize,
	cv_VectorOfGpuMat_shrink_to_fit,
	cv_VectorOfGpuMat_reserve, cv_VectorOfGpuMat_remove,
	cv_VectorOfGpuMat_swap, cv_VectorOfGpuMat_clear,
	cv_VectorOfGpuMat_get, cv_VectorOfGpuMat_set,
	cv_VectorOfGpuMat_push, cv_VectorOfGpuMat_insert,
}
vector_non_copy_or_bool! { clone core::GpuMat }

unsafe impl Send for core::Vector<core::GpuMat> {}

