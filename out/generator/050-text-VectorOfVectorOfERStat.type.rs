pub type VectorOfVectorOfERStat = core::Vector<core::Vector<crate::text::ERStat>>;

impl VectorOfVectorOfERStat {
	pub fn as_raw_VectorOfVectorOfERStat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<crate::text::ERStat>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfERStat_new, cv_VectorOfVectorOfERStat_delete,
	cv_VectorOfVectorOfERStat_len, cv_VectorOfVectorOfERStat_is_empty,
	cv_VectorOfVectorOfERStat_capacity, cv_VectorOfVectorOfERStat_resize,
	cv_VectorOfVectorOfERStat_shrink_to_fit,
	cv_VectorOfVectorOfERStat_reserve, cv_VectorOfVectorOfERStat_remove,
	cv_VectorOfVectorOfERStat_swap, cv_VectorOfVectorOfERStat_clear,
	cv_VectorOfVectorOfERStat_get, cv_VectorOfVectorOfERStat_set,
	cv_VectorOfVectorOfERStat_push, cv_VectorOfVectorOfERStat_insert,
}
vector_non_copy_or_bool! { core::Vector<crate::text::ERStat> }

unsafe impl Send for core::Vector<core::Vector<crate::text::ERStat>> {}

