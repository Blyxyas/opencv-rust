pub type VectorOfVectorOfKeyLine = core::Vector<core::Vector<crate::line_descriptor::KeyLine>>;

impl VectorOfVectorOfKeyLine {
	pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<crate::line_descriptor::KeyLine>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfKeyLine_new, cv_VectorOfVectorOfKeyLine_delete,
	cv_VectorOfVectorOfKeyLine_len, cv_VectorOfVectorOfKeyLine_is_empty,
	cv_VectorOfVectorOfKeyLine_capacity, cv_VectorOfVectorOfKeyLine_resize,
	cv_VectorOfVectorOfKeyLine_shrink_to_fit,
	cv_VectorOfVectorOfKeyLine_reserve, cv_VectorOfVectorOfKeyLine_remove,
	cv_VectorOfVectorOfKeyLine_swap, cv_VectorOfVectorOfKeyLine_clear,
	cv_VectorOfVectorOfKeyLine_get, cv_VectorOfVectorOfKeyLine_set,
	cv_VectorOfVectorOfKeyLine_push, cv_VectorOfVectorOfKeyLine_insert,
}
vector_non_copy_or_bool! { clone core::Vector<crate::line_descriptor::KeyLine> }

unsafe impl Send for core::Vector<core::Vector<crate::line_descriptor::KeyLine>> {}

