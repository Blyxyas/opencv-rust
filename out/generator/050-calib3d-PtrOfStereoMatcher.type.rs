pub type PtrOfStereoMatcher = core::Ptr<dyn crate::calib3d::StereoMatcher>;

ptr_extern! { dyn crate::calib3d::StereoMatcher,
	cv_PtrOfStereoMatcher_delete, cv_PtrOfStereoMatcher_get_inner_ptr, cv_PtrOfStereoMatcher_get_inner_ptr_mut
}

impl PtrOfStereoMatcher {
	#[inline] pub fn as_raw_PtrOfStereoMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoMatcherConst for PtrOfStereoMatcher {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcher for PtrOfStereoMatcher {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStereoMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStereoMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

