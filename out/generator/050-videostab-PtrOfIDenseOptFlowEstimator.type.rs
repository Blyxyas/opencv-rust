pub type PtrOfIDenseOptFlowEstimator = core::Ptr<dyn crate::videostab::IDenseOptFlowEstimator>;

ptr_extern! { dyn crate::videostab::IDenseOptFlowEstimator,
	cv_PtrOfIDenseOptFlowEstimator_delete, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut
}

impl PtrOfIDenseOptFlowEstimator {
	#[inline] pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::IDenseOptFlowEstimatorConst for PtrOfIDenseOptFlowEstimator {
	#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IDenseOptFlowEstimator for PtrOfIDenseOptFlowEstimator {
	#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

