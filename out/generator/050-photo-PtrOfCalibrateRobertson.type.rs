pub type PtrOfCalibrateRobertson = core::Ptr<dyn crate::photo::CalibrateRobertson>;

ptr_extern! { dyn crate::photo::CalibrateRobertson,
	cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr, cv_PtrOfCalibrateRobertson_get_inner_ptr_mut
}

impl PtrOfCalibrateRobertson {
	#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::CalibrateRobertsonConst for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::CalibrateCRFConst for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
	#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

