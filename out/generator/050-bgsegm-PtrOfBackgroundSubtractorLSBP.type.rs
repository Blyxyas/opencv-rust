pub type PtrOfBackgroundSubtractorLSBP = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorLSBP>;

ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorLSBP,
	cv_PtrOfBackgroundSubtractorLSBP_delete, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorLSBP {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorLSBP(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorLSBPConst for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_BackgroundSubtractorLSBP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorLSBP for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_mut_BackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorLSBP {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

