pub type PtrOfBackgroundSubtractorKNN = core::Ptr<dyn crate::video::BackgroundSubtractorKNN>;

ptr_extern! { dyn crate::video::BackgroundSubtractorKNN,
	cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut
}

impl PtrOfBackgroundSubtractorKNN {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::BackgroundSubtractorKNNConst for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

