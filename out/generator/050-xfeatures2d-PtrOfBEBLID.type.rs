pub type PtrOfBEBLID = core::Ptr<crate::xfeatures2d::BEBLID>;

ptr_extern! { crate::xfeatures2d::BEBLID,
	cv_PtrOfBEBLID_delete, cv_PtrOfBEBLID_get_inner_ptr, cv_PtrOfBEBLID_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::BEBLID, cv_PtrOfBEBLID_new }

impl PtrOfBEBLID {
	#[inline] pub fn as_raw_PtrOfBEBLID(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BEBLIDTraitConst for PtrOfBEBLID {
	#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BEBLIDTrait for PtrOfBEBLID {
	#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBEBLID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBEBLID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfBEBLID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfBEBLID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBEBLID, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfBEBLID_to_PtrOfFeature2D,
}

