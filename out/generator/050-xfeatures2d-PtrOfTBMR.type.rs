pub type PtrOfTBMR = core::Ptr<dyn crate::xfeatures2d::TBMR>;

ptr_extern! { dyn crate::xfeatures2d::TBMR,
	cv_PtrOfTBMR_delete, cv_PtrOfTBMR_get_inner_ptr, cv_PtrOfTBMR_get_inner_ptr_mut
}

impl PtrOfTBMR {
	#[inline] pub fn as_raw_PtrOfTBMR(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTBMR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::TBMRConst for PtrOfTBMR {
	#[inline] fn as_raw_TBMR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::TBMR for PtrOfTBMR {
	#[inline] fn as_raw_mut_TBMR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTBMR {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTBMR {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfTBMR {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfTBMR {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfTBMR, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfTBMR_to_PtrOfFeature2D,
}

impl crate::xfeatures2d::AffineFeature2DConst for PtrOfTBMR {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::AffineFeature2D for PtrOfTBMR {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

