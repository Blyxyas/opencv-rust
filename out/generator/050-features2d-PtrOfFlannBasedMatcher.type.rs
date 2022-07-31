pub type PtrOfFlannBasedMatcher = core::Ptr<crate::features2d::FlannBasedMatcher>;

ptr_extern! { crate::features2d::FlannBasedMatcher,
	cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr, cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::FlannBasedMatcher, cv_PtrOfFlannBasedMatcher_new }

impl PtrOfFlannBasedMatcher {
	#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::FlannBasedMatcherTraitConst for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::FlannBasedMatcherTrait for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::DescriptorMatcherConst for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::DescriptorMatcher for PtrOfFlannBasedMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

