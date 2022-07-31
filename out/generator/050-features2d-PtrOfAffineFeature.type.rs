pub type PtrOfAffineFeature = core::Ptr<dyn crate::features2d::AffineFeature>;

ptr_extern! { dyn crate::features2d::AffineFeature,
	cv_PtrOfAffineFeature_delete, cv_PtrOfAffineFeature_get_inner_ptr, cv_PtrOfAffineFeature_get_inner_ptr_mut
}

impl PtrOfAffineFeature {
	#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::AffineFeatureConst for PtrOfAffineFeature {
	#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::AffineFeature for PtrOfAffineFeature {
	#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAffineFeature {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAffineFeature {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfAffineFeature {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfAffineFeature {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAffineFeature, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfAffineFeature_to_PtrOfFeature2D,
}

