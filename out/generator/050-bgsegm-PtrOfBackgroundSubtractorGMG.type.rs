pub type PtrOfBackgroundSubtractorGMG = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorGMG>;

ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGMG,
	cv_PtrOfBackgroundSubtractorGMG_delete, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorGMG {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGMG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorGMGConst for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_mut_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGMG {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

