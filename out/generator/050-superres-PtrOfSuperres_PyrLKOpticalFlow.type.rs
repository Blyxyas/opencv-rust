pub type PtrOfSuperres_PyrLKOpticalFlow = core::Ptr<dyn crate::superres::Superres_PyrLKOpticalFlow>;

ptr_extern! { dyn crate::superres::Superres_PyrLKOpticalFlow,
	cv_PtrOfSuperres_PyrLKOpticalFlow_delete, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_PyrLKOpticalFlowConst for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_PyrLKOpticalFlow for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

