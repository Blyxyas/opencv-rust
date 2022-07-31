pub type PtrOfAgastFeatureDetector = core::Ptr<dyn crate::features2d::AgastFeatureDetector>;

ptr_extern! { dyn crate::features2d::AgastFeatureDetector,
	cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr, cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut
}

impl PtrOfAgastFeatureDetector {
	#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::AgastFeatureDetectorConst for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAgastFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfAgastFeatureDetector_to_PtrOfFeature2D,
}

