pub type PtrOfDISOpticalFlow = core::Ptr<dyn crate::video::DISOpticalFlow>;

ptr_extern! { dyn crate::video::DISOpticalFlow,
	cv_PtrOfDISOpticalFlow_delete, cv_PtrOfDISOpticalFlow_get_inner_ptr, cv_PtrOfDISOpticalFlow_get_inner_ptr_mut
}

impl PtrOfDISOpticalFlow {
	#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::DISOpticalFlowConst for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DISOpticalFlow for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfDISOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

