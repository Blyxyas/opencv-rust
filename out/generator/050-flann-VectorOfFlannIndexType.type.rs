pub type VectorOfFlannIndexType = core::Vector<crate::flann::FlannIndexType>;

impl VectorOfFlannIndexType {
	pub fn as_raw_VectorOfFlannIndexType(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
	cv_VectorOfFlannIndexType_new, cv_VectorOfFlannIndexType_delete,
	cv_VectorOfFlannIndexType_len, cv_VectorOfFlannIndexType_is_empty,
	cv_VectorOfFlannIndexType_capacity, cv_VectorOfFlannIndexType_resize,
	cv_VectorOfFlannIndexType_shrink_to_fit,
	cv_VectorOfFlannIndexType_reserve, cv_VectorOfFlannIndexType_remove,
	cv_VectorOfFlannIndexType_swap, cv_VectorOfFlannIndexType_clear,
	cv_VectorOfFlannIndexType_get, cv_VectorOfFlannIndexType_set,
	cv_VectorOfFlannIndexType_push, cv_VectorOfFlannIndexType_insert,
}
vector_copy_non_bool! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
	cv_VectorOfFlannIndexType_data, cv_VectorOfFlannIndexType_data_mut, cv_VectorOfFlannIndexType_from_slice,
	cv_VectorOfFlannIndexType_clone,
}

unsafe impl Send for core::Vector<crate::flann::FlannIndexType> {}

