pub type PtrOfStarDetector = core::Ptr<crate::xfeatures2d::StarDetector>;

ptr_extern! { crate::xfeatures2d::StarDetector,
	cv_PtrOfStarDetector_delete, cv_PtrOfStarDetector_get_inner_ptr, cv_PtrOfStarDetector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::StarDetector, cv_PtrOfStarDetector_new }

impl PtrOfStarDetector {
	#[inline] pub fn as_raw_PtrOfStarDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::StarDetectorTraitConst for PtrOfStarDetector {
	#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::StarDetectorTrait for PtrOfStarDetector {
	#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStarDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStarDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfStarDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfStarDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStarDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfStarDetector_to_PtrOfFeature2D,
}

