pub type VectorOfVectorOfDMatch = core::Vector<core::Vector<core::DMatch>>;

impl VectorOfVectorOfDMatch {
	pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::DMatch>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
	cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
	cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_resize,
	cv_VectorOfVectorOfDMatch_shrink_to_fit,
	cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
	cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
	cv_VectorOfVectorOfDMatch_get, cv_VectorOfVectorOfDMatch_set,
	cv_VectorOfVectorOfDMatch_push, cv_VectorOfVectorOfDMatch_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::DMatch> }

unsafe impl Send for core::Vector<core::Vector<core::DMatch>> {}

