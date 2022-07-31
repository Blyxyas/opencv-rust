pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr<dyn crate::video::BackgroundSubtractorMOG2>;

ptr_extern! { dyn crate::video::BackgroundSubtractorMOG2,
	cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorMOG2 {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::BackgroundSubtractorMOG2Const for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

