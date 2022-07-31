pub type VectorOfLinemod_Match = core::Vector<crate::rgbd::Linemod_Match>;

impl VectorOfLinemod_Match {
	pub fn as_raw_VectorOfLinemod_Match(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfLinemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::rgbd::Linemod_Match, *const c_void, *mut c_void,
	cv_VectorOfLinemod_Match_new, cv_VectorOfLinemod_Match_delete,
	cv_VectorOfLinemod_Match_len, cv_VectorOfLinemod_Match_is_empty,
	cv_VectorOfLinemod_Match_capacity, cv_VectorOfLinemod_Match_resize,
	cv_VectorOfLinemod_Match_shrink_to_fit,
	cv_VectorOfLinemod_Match_reserve, cv_VectorOfLinemod_Match_remove,
	cv_VectorOfLinemod_Match_swap, cv_VectorOfLinemod_Match_clear,
	cv_VectorOfLinemod_Match_get, cv_VectorOfLinemod_Match_set,
	cv_VectorOfLinemod_Match_push, cv_VectorOfLinemod_Match_insert,
}
vector_non_copy_or_bool! { crate::rgbd::Linemod_Match }

unsafe impl Send for core::Vector<crate::rgbd::Linemod_Match> {}

