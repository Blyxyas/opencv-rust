pub type VectorOfVectorOfKeyPoint = core::Vector<core::Vector<core::KeyPoint>>;

impl VectorOfVectorOfKeyPoint {
	pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::KeyPoint>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
	cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
	cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_resize,
	cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
	cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
	cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
	cv_VectorOfVectorOfKeyPoint_get, cv_VectorOfVectorOfKeyPoint_set,
	cv_VectorOfVectorOfKeyPoint_push, cv_VectorOfVectorOfKeyPoint_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::KeyPoint> }

unsafe impl Send for core::Vector<core::Vector<core::KeyPoint>> {}

