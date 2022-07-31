pub type PtrOfSparseRLOFOpticalFlow = core::Ptr<dyn crate::optflow::SparseRLOFOpticalFlow>;

ptr_extern! { dyn crate::optflow::SparseRLOFOpticalFlow,
	cv_PtrOfSparseRLOFOpticalFlow_delete, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSparseRLOFOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSparseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::SparseRLOFOpticalFlowConst for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::SparseRLOFOpticalFlow for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::SparseOpticalFlowConst for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlow for PtrOfSparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

