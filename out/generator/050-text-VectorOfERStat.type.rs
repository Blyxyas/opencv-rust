pub type VectorOfERStat = core::Vector<crate::text::ERStat>;

impl VectorOfERStat {
	pub fn as_raw_VectorOfERStat(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::text::ERStat, *const c_void, *mut c_void,
	cv_VectorOfERStat_new, cv_VectorOfERStat_delete,
	cv_VectorOfERStat_len, cv_VectorOfERStat_is_empty,
	cv_VectorOfERStat_capacity, cv_VectorOfERStat_resize,
	cv_VectorOfERStat_shrink_to_fit,
	cv_VectorOfERStat_reserve, cv_VectorOfERStat_remove,
	cv_VectorOfERStat_swap, cv_VectorOfERStat_clear,
	cv_VectorOfERStat_get, cv_VectorOfERStat_set,
	cv_VectorOfERStat_push, cv_VectorOfERStat_insert,
}
vector_non_copy_or_bool! { crate::text::ERStat }

unsafe impl Send for core::Vector<crate::text::ERStat> {}

