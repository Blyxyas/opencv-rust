pub type PtrOfFREAK = core::Ptr<crate::xfeatures2d::FREAK>;

ptr_extern! { crate::xfeatures2d::FREAK,
	cv_PtrOfFREAK_delete, cv_PtrOfFREAK_get_inner_ptr, cv_PtrOfFREAK_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::FREAK, cv_PtrOfFREAK_new }

impl PtrOfFREAK {
	#[inline] pub fn as_raw_PtrOfFREAK(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::FREAKTraitConst for PtrOfFREAK {
	#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::FREAKTrait for PtrOfFREAK {
	#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFREAK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFREAK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfFREAK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfFREAK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfFREAK, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfFREAK_to_PtrOfFeature2D,
}

