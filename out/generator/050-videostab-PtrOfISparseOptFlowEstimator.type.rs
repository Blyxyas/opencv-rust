pub type PtrOfISparseOptFlowEstimator = core::Ptr<dyn crate::videostab::ISparseOptFlowEstimator>;

ptr_extern! { dyn crate::videostab::ISparseOptFlowEstimator,
	cv_PtrOfISparseOptFlowEstimator_delete, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut
}

impl PtrOfISparseOptFlowEstimator {
	#[inline] pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ISparseOptFlowEstimatorConst for PtrOfISparseOptFlowEstimator {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimator for PtrOfISparseOptFlowEstimator {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

