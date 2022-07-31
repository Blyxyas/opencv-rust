pub type VectorOfString = core::Vector<String>;

impl VectorOfString {
	pub fn as_raw_VectorOfString(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfString(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { String, *const c_void, *mut c_void,
	cv_VectorOfString_new, cv_VectorOfString_delete,
	cv_VectorOfString_len, cv_VectorOfString_is_empty,
	cv_VectorOfString_capacity, cv_VectorOfString_resize,
	cv_VectorOfString_shrink_to_fit,
	cv_VectorOfString_reserve, cv_VectorOfString_remove,
	cv_VectorOfString_swap, cv_VectorOfString_clear,
	cv_VectorOfString_get, cv_VectorOfString_set,
	cv_VectorOfString_push, cv_VectorOfString_insert,
}
vector_non_copy_or_bool! { String }

unsafe impl Send for core::Vector<String> {}

