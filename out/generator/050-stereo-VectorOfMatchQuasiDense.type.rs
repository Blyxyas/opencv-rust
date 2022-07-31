pub type VectorOfMatchQuasiDense = core::Vector<crate::stereo::MatchQuasiDense>;

impl VectorOfMatchQuasiDense {
	pub fn as_raw_VectorOfMatchQuasiDense(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatchQuasiDense(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::stereo::MatchQuasiDense, *const c_void, *mut c_void,
	cv_VectorOfMatchQuasiDense_new, cv_VectorOfMatchQuasiDense_delete,
	cv_VectorOfMatchQuasiDense_len, cv_VectorOfMatchQuasiDense_is_empty,
	cv_VectorOfMatchQuasiDense_capacity, cv_VectorOfMatchQuasiDense_resize,
	cv_VectorOfMatchQuasiDense_shrink_to_fit,
	cv_VectorOfMatchQuasiDense_reserve, cv_VectorOfMatchQuasiDense_remove,
	cv_VectorOfMatchQuasiDense_swap, cv_VectorOfMatchQuasiDense_clear,
	cv_VectorOfMatchQuasiDense_get, cv_VectorOfMatchQuasiDense_set,
	cv_VectorOfMatchQuasiDense_push, cv_VectorOfMatchQuasiDense_insert,
}
vector_copy_non_bool! { crate::stereo::MatchQuasiDense, *const c_void, *mut c_void,
	cv_VectorOfMatchQuasiDense_data, cv_VectorOfMatchQuasiDense_data_mut, cv_VectorOfMatchQuasiDense_from_slice,
	cv_VectorOfMatchQuasiDense_clone,
}

unsafe impl Send for core::Vector<crate::stereo::MatchQuasiDense> {}

