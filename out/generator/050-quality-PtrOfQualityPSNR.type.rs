pub type PtrOfQualityPSNR = core::Ptr<crate::quality::QualityPSNR>;

ptr_extern! { crate::quality::QualityPSNR,
	cv_PtrOfQualityPSNR_delete, cv_PtrOfQualityPSNR_get_inner_ptr, cv_PtrOfQualityPSNR_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::quality::QualityPSNR, cv_PtrOfQualityPSNR_new }

impl PtrOfQualityPSNR {
	#[inline] pub fn as_raw_PtrOfQualityPSNR(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityPSNR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityPSNRTraitConst for PtrOfQualityPSNR {
	#[inline] fn as_raw_QualityPSNR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityPSNRTrait for PtrOfQualityPSNR {
	#[inline] fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQualityPSNR {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQualityPSNR {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::quality::QualityBaseConst for PtrOfQualityPSNR {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBase for PtrOfQualityPSNR {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

