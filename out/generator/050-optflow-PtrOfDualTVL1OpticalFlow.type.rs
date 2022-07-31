pub type PtrOfDualTVL1OpticalFlow = core::Ptr<dyn crate::optflow::DualTVL1OpticalFlow>;

ptr_extern! { dyn crate::optflow::DualTVL1OpticalFlow,
	cv_PtrOfDualTVL1OpticalFlow_delete, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut
}

impl PtrOfDualTVL1OpticalFlow {
	#[inline] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::DualTVL1OpticalFlowConst for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

