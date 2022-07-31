pub type PtrOfDenseRLOFOpticalFlow = core::Ptr<dyn crate::optflow::DenseRLOFOpticalFlow>;

ptr_extern! { dyn crate::optflow::DenseRLOFOpticalFlow,
	cv_PtrOfDenseRLOFOpticalFlow_delete, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr_mut
}

impl PtrOfDenseRLOFOpticalFlow {
	#[inline] pub fn as_raw_PtrOfDenseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::DenseRLOFOpticalFlowConst for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::DenseRLOFOpticalFlow for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfDenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

