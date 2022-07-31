pub type PtrOfFastFeatureDetector = core::Ptr<dyn crate::features2d::FastFeatureDetector>;

ptr_extern! { dyn crate::features2d::FastFeatureDetector,
	cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr, cv_PtrOfFastFeatureDetector_get_inner_ptr_mut
}

impl PtrOfFastFeatureDetector {
	#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::FastFeatureDetectorConst for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfFastFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfFastFeatureDetector_to_PtrOfFeature2D,
}

