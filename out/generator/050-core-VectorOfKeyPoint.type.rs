pub type VectorOfKeyPoint = core::Vector<core::KeyPoint>;

impl VectorOfKeyPoint {
	pub fn as_raw_VectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::KeyPoint, *const c_void, *mut c_void,
	cv_VectorOfKeyPoint_new, cv_VectorOfKeyPoint_delete,
	cv_VectorOfKeyPoint_len, cv_VectorOfKeyPoint_is_empty,
	cv_VectorOfKeyPoint_capacity, cv_VectorOfKeyPoint_resize,
	cv_VectorOfKeyPoint_shrink_to_fit,
	cv_VectorOfKeyPoint_reserve, cv_VectorOfKeyPoint_remove,
	cv_VectorOfKeyPoint_swap, cv_VectorOfKeyPoint_clear,
	cv_VectorOfKeyPoint_get, cv_VectorOfKeyPoint_set,
	cv_VectorOfKeyPoint_push, cv_VectorOfKeyPoint_insert,
}
vector_copy_non_bool! { core::KeyPoint, *const c_void, *mut c_void,
	cv_VectorOfKeyPoint_data, cv_VectorOfKeyPoint_data_mut, cv_VectorOfKeyPoint_from_slice,
	cv_VectorOfKeyPoint_clone,
}

unsafe impl Send for core::Vector<core::KeyPoint> {}

