pub type PtrOfGFTTDetector = core::Ptr<dyn crate::features2d::GFTTDetector>;

ptr_extern! { dyn crate::features2d::GFTTDetector,
	cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr, cv_PtrOfGFTTDetector_get_inner_ptr_mut
}

impl PtrOfGFTTDetector {
	#[inline] pub fn as_raw_PtrOfGFTTDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::GFTTDetectorConst for PtrOfGFTTDetector {
	#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
	#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGFTTDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGFTTDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfGFTTDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfGFTTDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfGFTTDetector_to_PtrOfFeature2D,
}

