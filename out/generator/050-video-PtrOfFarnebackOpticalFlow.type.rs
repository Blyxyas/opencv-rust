pub type PtrOfFarnebackOpticalFlow = core::Ptr<dyn crate::video::FarnebackOpticalFlow>;

ptr_extern! { dyn crate::video::FarnebackOpticalFlow,
	cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut
}

impl PtrOfFarnebackOpticalFlow {
	#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::FarnebackOpticalFlowConst for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

