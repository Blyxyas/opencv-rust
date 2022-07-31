pub type PtrOfPCTSignaturesSQFD = core::Ptr<dyn crate::xfeatures2d::PCTSignaturesSQFD>;

ptr_extern! { dyn crate::xfeatures2d::PCTSignaturesSQFD,
	cv_PtrOfPCTSignaturesSQFD_delete, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr_mut
}

impl PtrOfPCTSignaturesSQFD {
	#[inline] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::PCTSignaturesSQFDConst for PtrOfPCTSignaturesSQFD {
	#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
	#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPCTSignaturesSQFD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

