pub type VectorOfDMatch = core::Vector<core::DMatch>;

impl VectorOfDMatch {
	pub fn as_raw_VectorOfDMatch(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::DMatch, *const c_void, *mut c_void,
	cv_VectorOfDMatch_new, cv_VectorOfDMatch_delete,
	cv_VectorOfDMatch_len, cv_VectorOfDMatch_is_empty,
	cv_VectorOfDMatch_capacity, cv_VectorOfDMatch_resize,
	cv_VectorOfDMatch_shrink_to_fit,
	cv_VectorOfDMatch_reserve, cv_VectorOfDMatch_remove,
	cv_VectorOfDMatch_swap, cv_VectorOfDMatch_clear,
	cv_VectorOfDMatch_get, cv_VectorOfDMatch_set,
	cv_VectorOfDMatch_push, cv_VectorOfDMatch_insert,
}
vector_copy_non_bool! { core::DMatch, *const c_void, *mut c_void,
	cv_VectorOfDMatch_data, cv_VectorOfDMatch_data_mut, cv_VectorOfDMatch_from_slice,
	cv_VectorOfDMatch_clone,
}

unsafe impl Send for core::Vector<core::DMatch> {}

