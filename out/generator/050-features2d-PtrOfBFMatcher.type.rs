pub type PtrOfBFMatcher = core::Ptr<crate::features2d::BFMatcher>;

ptr_extern! { crate::features2d::BFMatcher,
	cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr, cv_PtrOfBFMatcher_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrOfBFMatcher_new }

impl PtrOfBFMatcher {
	#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BFMatcherTraitConst for PtrOfBFMatcher {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::BFMatcherTrait for PtrOfBFMatcher {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBFMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBFMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::DescriptorMatcherConst for PtrOfBFMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::DescriptorMatcher for PtrOfBFMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

