pub type VectorOfLinemod_Feature = core::Vector<crate::rgbd::Linemod_Feature>;

impl VectorOfLinemod_Feature {
	pub fn as_raw_VectorOfLinemod_Feature(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfLinemod_Feature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
	cv_VectorOfLinemod_Feature_new, cv_VectorOfLinemod_Feature_delete,
	cv_VectorOfLinemod_Feature_len, cv_VectorOfLinemod_Feature_is_empty,
	cv_VectorOfLinemod_Feature_capacity, cv_VectorOfLinemod_Feature_resize,
	cv_VectorOfLinemod_Feature_shrink_to_fit,
	cv_VectorOfLinemod_Feature_reserve, cv_VectorOfLinemod_Feature_remove,
	cv_VectorOfLinemod_Feature_swap, cv_VectorOfLinemod_Feature_clear,
	cv_VectorOfLinemod_Feature_get, cv_VectorOfLinemod_Feature_set,
	cv_VectorOfLinemod_Feature_push, cv_VectorOfLinemod_Feature_insert,
}
vector_copy_non_bool! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
	cv_VectorOfLinemod_Feature_data, cv_VectorOfLinemod_Feature_data_mut, cv_VectorOfLinemod_Feature_from_slice,
	cv_VectorOfLinemod_Feature_clone,
}

unsafe impl Send for core::Vector<crate::rgbd::Linemod_Feature> {}

