pub type PtrOfSuperres_BroxOpticalFlow = core::Ptr<dyn crate::superres::Superres_BroxOpticalFlow>;

ptr_extern! { dyn crate::superres::Superres_BroxOpticalFlow,
	cv_PtrOfSuperres_BroxOpticalFlow_delete, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSuperres_BroxOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_BroxOpticalFlowConst for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_BroxOpticalFlow for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_BroxOpticalFlow {
	#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

