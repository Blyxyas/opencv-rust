pub type VectorOfDetail_ImageFeatures = core::Vector<crate::stitching::Detail_ImageFeatures>;

impl VectorOfDetail_ImageFeatures {
	pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_ImageFeatures, *const c_void, *mut c_void,
	cv_VectorOfDetail_ImageFeatures_new, cv_VectorOfDetail_ImageFeatures_delete,
	cv_VectorOfDetail_ImageFeatures_len, cv_VectorOfDetail_ImageFeatures_is_empty,
	cv_VectorOfDetail_ImageFeatures_capacity, cv_VectorOfDetail_ImageFeatures_resize,
	cv_VectorOfDetail_ImageFeatures_shrink_to_fit,
	cv_VectorOfDetail_ImageFeatures_reserve, cv_VectorOfDetail_ImageFeatures_remove,
	cv_VectorOfDetail_ImageFeatures_swap, cv_VectorOfDetail_ImageFeatures_clear,
	cv_VectorOfDetail_ImageFeatures_get, cv_VectorOfDetail_ImageFeatures_set,
	cv_VectorOfDetail_ImageFeatures_push, cv_VectorOfDetail_ImageFeatures_insert,
}
vector_non_copy_or_bool! { crate::stitching::Detail_ImageFeatures }

unsafe impl Send for core::Vector<crate::stitching::Detail_ImageFeatures> {}

