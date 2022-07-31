pub type VectorOfKeyLine = core::Vector<crate::line_descriptor::KeyLine>;

impl VectorOfKeyLine {
	pub fn as_raw_VectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
	cv_VectorOfKeyLine_new, cv_VectorOfKeyLine_delete,
	cv_VectorOfKeyLine_len, cv_VectorOfKeyLine_is_empty,
	cv_VectorOfKeyLine_capacity, cv_VectorOfKeyLine_resize,
	cv_VectorOfKeyLine_shrink_to_fit,
	cv_VectorOfKeyLine_reserve, cv_VectorOfKeyLine_remove,
	cv_VectorOfKeyLine_swap, cv_VectorOfKeyLine_clear,
	cv_VectorOfKeyLine_get, cv_VectorOfKeyLine_set,
	cv_VectorOfKeyLine_push, cv_VectorOfKeyLine_insert,
}
vector_copy_non_bool! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
	cv_VectorOfKeyLine_data, cv_VectorOfKeyLine_data_mut, cv_VectorOfKeyLine_from_slice,
	cv_VectorOfKeyLine_clone,
}

unsafe impl Send for core::Vector<crate::line_descriptor::KeyLine> {}

