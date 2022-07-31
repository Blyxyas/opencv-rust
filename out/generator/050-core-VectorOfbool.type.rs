pub type VectorOfbool = core::Vector<bool>;

impl VectorOfbool {
	pub fn as_raw_VectorOfbool(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfbool(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { bool, *const c_void, *mut c_void,
	cv_VectorOfbool_new, cv_VectorOfbool_delete,
	cv_VectorOfbool_len, cv_VectorOfbool_is_empty,
	cv_VectorOfbool_capacity, cv_VectorOfbool_resize,
	cv_VectorOfbool_shrink_to_fit,
	cv_VectorOfbool_reserve, cv_VectorOfbool_remove,
	cv_VectorOfbool_swap, cv_VectorOfbool_clear,
	cv_VectorOfbool_get, cv_VectorOfbool_set,
	cv_VectorOfbool_push, cv_VectorOfbool_insert,
}
vector_non_copy_or_bool! { clone bool }

unsafe impl Send for core::Vector<bool> {}

