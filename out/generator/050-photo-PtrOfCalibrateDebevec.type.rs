pub type PtrOfCalibrateDebevec = core::Ptr<dyn crate::photo::CalibrateDebevec>;

ptr_extern! { dyn crate::photo::CalibrateDebevec,
	cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr, cv_PtrOfCalibrateDebevec_get_inner_ptr_mut
}

impl PtrOfCalibrateDebevec {
	#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::CalibrateDebevecConst for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::CalibrateCRFConst for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
	#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

