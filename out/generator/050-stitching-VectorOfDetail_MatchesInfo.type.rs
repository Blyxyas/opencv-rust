pub type VectorOfDetail_MatchesInfo = core::Vector<crate::stitching::Detail_MatchesInfo>;

impl VectorOfDetail_MatchesInfo {
	pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_MatchesInfo, *const c_void, *mut c_void,
	cv_VectorOfDetail_MatchesInfo_new, cv_VectorOfDetail_MatchesInfo_delete,
	cv_VectorOfDetail_MatchesInfo_len, cv_VectorOfDetail_MatchesInfo_is_empty,
	cv_VectorOfDetail_MatchesInfo_capacity, cv_VectorOfDetail_MatchesInfo_resize,
	cv_VectorOfDetail_MatchesInfo_shrink_to_fit,
	cv_VectorOfDetail_MatchesInfo_reserve, cv_VectorOfDetail_MatchesInfo_remove,
	cv_VectorOfDetail_MatchesInfo_swap, cv_VectorOfDetail_MatchesInfo_clear,
	cv_VectorOfDetail_MatchesInfo_get, cv_VectorOfDetail_MatchesInfo_set,
	cv_VectorOfDetail_MatchesInfo_push, cv_VectorOfDetail_MatchesInfo_insert,
}
vector_non_copy_or_bool! { crate::stitching::Detail_MatchesInfo }

unsafe impl Send for core::Vector<crate::stitching::Detail_MatchesInfo> {}

