pub type PtrOfLUCID = core::Ptr<crate::xfeatures2d::LUCID>;

ptr_extern! { crate::xfeatures2d::LUCID,
	cv_PtrOfLUCID_delete, cv_PtrOfLUCID_get_inner_ptr, cv_PtrOfLUCID_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::LUCID, cv_PtrOfLUCID_new }

impl PtrOfLUCID {
	#[inline] pub fn as_raw_PtrOfLUCID(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LUCIDTraitConst for PtrOfLUCID {
	#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::LUCIDTrait for PtrOfLUCID {
	#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLUCID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLUCID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfLUCID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfLUCID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfLUCID, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfLUCID_to_PtrOfFeature2D,
}

