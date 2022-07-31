pub type PtrOfDAISY = core::Ptr<dyn crate::xfeatures2d::DAISY>;

ptr_extern! { dyn crate::xfeatures2d::DAISY,
	cv_PtrOfDAISY_delete, cv_PtrOfDAISY_get_inner_ptr, cv_PtrOfDAISY_get_inner_ptr_mut
}

impl PtrOfDAISY {
	#[inline] pub fn as_raw_PtrOfDAISY(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDAISY(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::DAISYConst for PtrOfDAISY {
	#[inline] fn as_raw_DAISY(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::DAISY for PtrOfDAISY {
	#[inline] fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDAISY {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDAISY {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfDAISY {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfDAISY {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDAISY, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfDAISY_to_PtrOfFeature2D,
}

