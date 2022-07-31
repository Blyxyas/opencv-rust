pub type VectorOfTarget = core::Vector<crate::dnn::Target>;

impl VectorOfTarget {
	pub fn as_raw_VectorOfTarget(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfTarget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::dnn::Target, *const c_void, *mut c_void,
	cv_VectorOfTarget_new, cv_VectorOfTarget_delete,
	cv_VectorOfTarget_len, cv_VectorOfTarget_is_empty,
	cv_VectorOfTarget_capacity, cv_VectorOfTarget_resize,
	cv_VectorOfTarget_shrink_to_fit,
	cv_VectorOfTarget_reserve, cv_VectorOfTarget_remove,
	cv_VectorOfTarget_swap, cv_VectorOfTarget_clear,
	cv_VectorOfTarget_get, cv_VectorOfTarget_set,
	cv_VectorOfTarget_push, cv_VectorOfTarget_insert,
}
vector_copy_non_bool! { crate::dnn::Target, *const c_void, *mut c_void,
	cv_VectorOfTarget_data, cv_VectorOfTarget_data_mut, cv_VectorOfTarget_from_slice,
	cv_VectorOfTarget_clone,
}

unsafe impl Send for core::Vector<crate::dnn::Target> {}

