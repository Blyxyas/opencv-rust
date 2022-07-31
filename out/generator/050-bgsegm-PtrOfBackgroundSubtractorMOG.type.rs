pub type PtrOfBackgroundSubtractorMOG = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorMOG>;

ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorMOG,
	cv_PtrOfBackgroundSubtractorMOG_delete, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorMOG {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorMOGConst for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

