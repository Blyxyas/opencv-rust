pub type PtrOfSuperres_DualTVL1OpticalFlow = core::Ptr<dyn crate::superres::Superres_DualTVL1OpticalFlow>;

ptr_extern! { dyn crate::superres::Superres_DualTVL1OpticalFlow,
	cv_PtrOfSuperres_DualTVL1OpticalFlow_delete, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr_mut
}

impl PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_DualTVL1OpticalFlowConst for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DualTVL1OpticalFlow for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

