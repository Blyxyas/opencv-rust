pub type PtrOfBoostDesc = core::Ptr<dyn crate::xfeatures2d::BoostDesc>;

ptr_extern! { dyn crate::xfeatures2d::BoostDesc,
	cv_PtrOfBoostDesc_delete, cv_PtrOfBoostDesc_get_inner_ptr, cv_PtrOfBoostDesc_get_inner_ptr_mut
}

impl PtrOfBoostDesc {
	#[inline] pub fn as_raw_PtrOfBoostDesc(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BoostDescConst for PtrOfBoostDesc {
	#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BoostDesc for PtrOfBoostDesc {
	#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBoostDesc {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBoostDesc {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfBoostDesc {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfBoostDesc {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBoostDesc, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfBoostDesc_to_PtrOfFeature2D,
}

