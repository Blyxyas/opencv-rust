pub type PtrOfMSDDetector = core::Ptr<crate::xfeatures2d::MSDDetector>;

ptr_extern! { crate::xfeatures2d::MSDDetector,
	cv_PtrOfMSDDetector_delete, cv_PtrOfMSDDetector_get_inner_ptr, cv_PtrOfMSDDetector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::MSDDetector, cv_PtrOfMSDDetector_new }

impl PtrOfMSDDetector {
	#[inline] pub fn as_raw_PtrOfMSDDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::MSDDetectorTraitConst for PtrOfMSDDetector {
	#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::MSDDetectorTrait for PtrOfMSDDetector {
	#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfMSDDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfMSDDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMSDDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfMSDDetector_to_PtrOfFeature2D,
}

