pub type PtrOfPCTSignatures = core::Ptr<dyn crate::xfeatures2d::PCTSignatures>;

ptr_extern! { dyn crate::xfeatures2d::PCTSignatures,
	cv_PtrOfPCTSignatures_delete, cv_PtrOfPCTSignatures_get_inner_ptr, cv_PtrOfPCTSignatures_get_inner_ptr_mut
}

impl PtrOfPCTSignatures {
	#[inline] pub fn as_raw_PtrOfPCTSignatures(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::PCTSignaturesConst for PtrOfPCTSignatures {
	#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
	#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPCTSignatures {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPCTSignatures {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

