pub type PtrOfSURF = core::Ptr<dyn crate::xfeatures2d::SURF>;

ptr_extern! { dyn crate::xfeatures2d::SURF,
	cv_PtrOfSURF_delete, cv_PtrOfSURF_get_inner_ptr, cv_PtrOfSURF_get_inner_ptr_mut
}

impl PtrOfSURF {
	#[inline] pub fn as_raw_PtrOfSURF(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSURF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::SURFConst for PtrOfSURF {
	#[inline] fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::SURF for PtrOfSURF {
	#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSURF {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSURF {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfSURF {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfSURF {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSURF, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfSURF_to_PtrOfFeature2D,
}

