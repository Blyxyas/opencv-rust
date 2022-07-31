pub type PtrOfDescriptorMatcher = core::Ptr<dyn crate::features2d::DescriptorMatcher>;

ptr_extern! { dyn crate::features2d::DescriptorMatcher,
	cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr, cv_PtrOfDescriptorMatcher_get_inner_ptr_mut
}

impl PtrOfDescriptorMatcher {
	#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::DescriptorMatcherConst for PtrOfDescriptorMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDescriptorMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

