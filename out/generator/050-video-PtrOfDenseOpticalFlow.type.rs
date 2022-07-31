pub type PtrOfDenseOpticalFlow = core::Ptr<dyn crate::video::DenseOpticalFlow>;

ptr_extern! { dyn crate::video::DenseOpticalFlow,
	cv_PtrOfDenseOpticalFlow_delete, cv_PtrOfDenseOpticalFlow_get_inner_ptr, cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut
}

impl PtrOfDenseOpticalFlow {
	#[inline] pub fn as_raw_PtrOfDenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfDenseOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfDenseOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDenseOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDenseOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

