pub type PtrOfSparsePyrLkOptFlowEstimator = core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator>;

ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimator,
	cv_PtrOfSparsePyrLkOptFlowEstimator_delete, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimator, cv_PtrOfSparsePyrLkOptFlowEstimator_new }

impl PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ISparseOptFlowEstimatorConst for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimator for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSparsePyrLkOptFlowEstimator, core::Ptr<dyn crate::videostab::ISparseOptFlowEstimator>,
	cv_PtrOfSparsePyrLkOptFlowEstimator_to_PtrOfISparseOptFlowEstimator,
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PtrOfSparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

