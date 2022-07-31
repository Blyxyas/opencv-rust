pub type PtrOfSuperres_FarnebackOpticalFlow = core::Ptr<dyn crate::superres::Superres_FarnebackOpticalFlow>;

ptr_extern! { dyn crate::superres::Superres_FarnebackOpticalFlow,
	cv_PtrOfSuperres_FarnebackOpticalFlow_delete, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_FarnebackOpticalFlowConst for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_FarnebackOpticalFlow for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

