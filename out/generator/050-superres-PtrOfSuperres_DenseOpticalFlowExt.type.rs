pub type PtrOfSuperres_DenseOpticalFlowExt = core::Ptr<dyn crate::superres::Superres_DenseOpticalFlowExt>;

ptr_extern! { dyn crate::superres::Superres_DenseOpticalFlowExt,
	cv_PtrOfSuperres_DenseOpticalFlowExt_delete, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr_mut
}

impl PtrOfSuperres_DenseOpticalFlowExt {
	#[inline] pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_DenseOpticalFlowExt {
	#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DenseOpticalFlowExt {
	#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_DenseOpticalFlowExt {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_DenseOpticalFlowExt {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

