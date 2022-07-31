pub type PtrOfDetail_FeaturesMatcher = core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>;

ptr_extern! { dyn crate::stitching::Detail_FeaturesMatcher,
	cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut
}

impl PtrOfDetail_FeaturesMatcher {
	#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcherConst for PtrOfDetail_FeaturesMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_FeaturesMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

