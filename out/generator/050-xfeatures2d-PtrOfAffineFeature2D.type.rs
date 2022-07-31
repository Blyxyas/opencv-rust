pub type PtrOfAffineFeature2D = core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>;

ptr_extern! { dyn crate::xfeatures2d::AffineFeature2D,
	cv_PtrOfAffineFeature2D_delete, cv_PtrOfAffineFeature2D_get_inner_ptr, cv_PtrOfAffineFeature2D_get_inner_ptr_mut
}

impl PtrOfAffineFeature2D {
	#[inline] pub fn as_raw_PtrOfAffineFeature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::AffineFeature2DConst for PtrOfAffineFeature2D {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::AffineFeature2D for PtrOfAffineFeature2D {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAffineFeature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAffineFeature2D {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfAffineFeature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfAffineFeature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAffineFeature2D, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfAffineFeature2D_to_PtrOfFeature2D,
}

