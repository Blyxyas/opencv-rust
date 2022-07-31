pub type PtrOfQualityGMSD = core::Ptr<crate::quality::QualityGMSD>;

ptr_extern! { crate::quality::QualityGMSD,
	cv_PtrOfQualityGMSD_delete, cv_PtrOfQualityGMSD_get_inner_ptr, cv_PtrOfQualityGMSD_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::quality::QualityGMSD, cv_PtrOfQualityGMSD_new }

impl PtrOfQualityGMSD {
	#[inline] pub fn as_raw_PtrOfQualityGMSD(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityGMSD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityGMSDTraitConst for PtrOfQualityGMSD {
	#[inline] fn as_raw_QualityGMSD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityGMSDTrait for PtrOfQualityGMSD {
	#[inline] fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQualityGMSD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQualityGMSD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::quality::QualityBaseConst for PtrOfQualityGMSD {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBase for PtrOfQualityGMSD {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

