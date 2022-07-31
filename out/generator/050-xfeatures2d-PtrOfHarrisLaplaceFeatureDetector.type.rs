pub type PtrOfHarrisLaplaceFeatureDetector = core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>;

ptr_extern! { crate::xfeatures2d::HarrisLaplaceFeatureDetector,
	cv_PtrOfHarrisLaplaceFeatureDetector_delete, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::HarrisLaplaceFeatureDetector, cv_PtrOfHarrisLaplaceFeatureDetector_new }

impl PtrOfHarrisLaplaceFeatureDetector {
	#[inline] pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfHarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfHarrisLaplaceFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfHarrisLaplaceFeatureDetector_to_PtrOfFeature2D,
}

