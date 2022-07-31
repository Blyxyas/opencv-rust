pub type PtrOfBinaryDescriptorMatcher = core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>;

ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
	cv_PtrOfBinaryDescriptorMatcher_delete, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptorMatcher, cv_PtrOfBinaryDescriptorMatcher_new }

impl PtrOfBinaryDescriptorMatcher {
	#[inline] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for PtrOfBinaryDescriptorMatcher {
	#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTrait for PtrOfBinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBinaryDescriptorMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

