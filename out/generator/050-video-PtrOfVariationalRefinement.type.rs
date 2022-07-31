pub type PtrOfVariationalRefinement = core::Ptr<dyn crate::video::VariationalRefinement>;

ptr_extern! { dyn crate::video::VariationalRefinement,
	cv_PtrOfVariationalRefinement_delete, cv_PtrOfVariationalRefinement_get_inner_ptr, cv_PtrOfVariationalRefinement_get_inner_ptr_mut
}

impl PtrOfVariationalRefinement {
	#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::VariationalRefinementConst for PtrOfVariationalRefinement {
	#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::VariationalRefinement for PtrOfVariationalRefinement {
	#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfVariationalRefinement {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfVariationalRefinement {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::DenseOpticalFlowConst for PtrOfVariationalRefinement {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlow for PtrOfVariationalRefinement {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

