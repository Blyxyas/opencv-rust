pub type PtrOfDetail_BestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>;

ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
	cv_PtrOfDetail_BestOf2NearestMatcher_delete, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrOfDetail_BestOf2NearestMatcher_new }

impl PtrOfDetail_BestOf2NearestMatcher {
	#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for PtrOfDetail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for PtrOfDetail_BestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcherConst for PtrOfDetail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_BestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BestOf2NearestMatcher, core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>,
	cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher,
}

