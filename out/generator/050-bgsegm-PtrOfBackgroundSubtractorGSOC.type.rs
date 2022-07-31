pub type PtrOfBackgroundSubtractorGSOC = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorGSOC>;

ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGSOC,
	cv_PtrOfBackgroundSubtractorGSOC_delete, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorGSOC {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGSOC(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorGSOCConst for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_BackgroundSubtractorGSOC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorGSOC for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_mut_BackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGSOC {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

