pub type VectorOfLinemod_Template = core::Vector<crate::rgbd::Linemod_Template>;

impl VectorOfLinemod_Template {
	pub fn as_raw_VectorOfLinemod_Template(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfLinemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::rgbd::Linemod_Template, *const c_void, *mut c_void,
	cv_VectorOfLinemod_Template_new, cv_VectorOfLinemod_Template_delete,
	cv_VectorOfLinemod_Template_len, cv_VectorOfLinemod_Template_is_empty,
	cv_VectorOfLinemod_Template_capacity, cv_VectorOfLinemod_Template_resize,
	cv_VectorOfLinemod_Template_shrink_to_fit,
	cv_VectorOfLinemod_Template_reserve, cv_VectorOfLinemod_Template_remove,
	cv_VectorOfLinemod_Template_swap, cv_VectorOfLinemod_Template_clear,
	cv_VectorOfLinemod_Template_get, cv_VectorOfLinemod_Template_set,
	cv_VectorOfLinemod_Template_push, cv_VectorOfLinemod_Template_insert,
}
vector_non_copy_or_bool! { crate::rgbd::Linemod_Template }

unsafe impl Send for core::Vector<crate::rgbd::Linemod_Template> {}

