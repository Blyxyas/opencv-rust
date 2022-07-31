pub type PtrOfLearningBasedWB = core::Ptr<dyn crate::xphoto::LearningBasedWB>;

ptr_extern! { dyn crate::xphoto::LearningBasedWB,
	cv_PtrOfLearningBasedWB_delete, cv_PtrOfLearningBasedWB_get_inner_ptr, cv_PtrOfLearningBasedWB_get_inner_ptr_mut
}

impl PtrOfLearningBasedWB {
	#[inline] pub fn as_raw_PtrOfLearningBasedWB(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::LearningBasedWBConst for PtrOfLearningBasedWB {
	#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
	#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLearningBasedWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLearningBasedWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerConst for PtrOfLearningBasedWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

