pub type VectorOfVectorOfRange = core::Vector<core::Vector<core::Range>>;

impl VectorOfVectorOfRange {
	pub fn as_raw_VectorOfVectorOfRange(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Range>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfRange_new, cv_VectorOfVectorOfRange_delete,
	cv_VectorOfVectorOfRange_len, cv_VectorOfVectorOfRange_is_empty,
	cv_VectorOfVectorOfRange_capacity, cv_VectorOfVectorOfRange_resize,
	cv_VectorOfVectorOfRange_shrink_to_fit,
	cv_VectorOfVectorOfRange_reserve, cv_VectorOfVectorOfRange_remove,
	cv_VectorOfVectorOfRange_swap, cv_VectorOfVectorOfRange_clear,
	cv_VectorOfVectorOfRange_get, cv_VectorOfVectorOfRange_set,
	cv_VectorOfVectorOfRange_push, cv_VectorOfVectorOfRange_insert,
}
vector_non_copy_or_bool! { core::Vector<core::Range> }

unsafe impl Send for core::Vector<core::Vector<core::Range>> {}

