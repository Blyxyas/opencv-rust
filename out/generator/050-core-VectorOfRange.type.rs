pub type VectorOfRange = core::Vector<core::Range>;

impl VectorOfRange {
	pub fn as_raw_VectorOfRange(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Range, *const c_void, *mut c_void,
	cv_VectorOfRange_new, cv_VectorOfRange_delete,
	cv_VectorOfRange_len, cv_VectorOfRange_is_empty,
	cv_VectorOfRange_capacity, cv_VectorOfRange_resize,
	cv_VectorOfRange_shrink_to_fit,
	cv_VectorOfRange_reserve, cv_VectorOfRange_remove,
	cv_VectorOfRange_swap, cv_VectorOfRange_clear,
	cv_VectorOfRange_get, cv_VectorOfRange_set,
	cv_VectorOfRange_push, cv_VectorOfRange_insert,
}
vector_non_copy_or_bool! { core::Range }

unsafe impl Send for core::Vector<core::Range> {}

