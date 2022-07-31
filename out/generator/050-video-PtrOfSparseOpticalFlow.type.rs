pub type PtrOfSparseOpticalFlow = core::Ptr<dyn crate::video::SparseOpticalFlow>;

ptr_extern! { dyn crate::video::SparseOpticalFlow,
	cv_PtrOfSparseOpticalFlow_delete, cv_PtrOfSparseOpticalFlow_get_inner_ptr, cv_PtrOfSparseOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSparseOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::SparseOpticalFlowConst for PtrOfSparseOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlow for PtrOfSparseOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSparseOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSparseOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

