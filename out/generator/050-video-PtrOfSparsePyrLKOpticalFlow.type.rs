pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow>;

ptr_extern! { dyn crate::video::SparsePyrLKOpticalFlow,
	cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut
}

impl PtrOfSparsePyrLKOpticalFlow {
	#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::SparsePyrLKOpticalFlowConst for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::SparseOpticalFlowConst for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

