pub type PtrOfBRISK = core::Ptr<crate::features2d::BRISK>;

ptr_extern! { crate::features2d::BRISK,
	cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr, cv_PtrOfBRISK_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::BRISK, cv_PtrOfBRISK_new }

impl PtrOfBRISK {
	#[inline] pub fn as_raw_PtrOfBRISK(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BRISKTraitConst for PtrOfBRISK {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::BRISKTrait for PtrOfBRISK {
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBRISK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBRISK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfBRISK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfBRISK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBRISK, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfBRISK_to_PtrOfFeature2D,
}

