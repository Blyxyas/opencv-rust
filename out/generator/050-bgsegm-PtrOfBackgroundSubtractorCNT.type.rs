pub type PtrOfBackgroundSubtractorCNT = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorCNT>;

ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorCNT,
	cv_PtrOfBackgroundSubtractorCNT_delete, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorCNT {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorCNT(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorCNT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::BackgroundSubtractorCNTConst for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_BackgroundSubtractorCNT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::BackgroundSubtractorCNT for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_mut_BackgroundSubtractorCNT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorCNT {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

