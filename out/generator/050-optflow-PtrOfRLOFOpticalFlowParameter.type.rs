pub type PtrOfRLOFOpticalFlowParameter = core::Ptr<crate::optflow::RLOFOpticalFlowParameter>;

ptr_extern! { crate::optflow::RLOFOpticalFlowParameter,
	cv_PtrOfRLOFOpticalFlowParameter_delete, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::optflow::RLOFOpticalFlowParameter, cv_PtrOfRLOFOpticalFlowParameter_new }

impl PtrOfRLOFOpticalFlowParameter {
	#[inline] pub fn as_raw_PtrOfRLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::RLOFOpticalFlowParameterTraitConst for PtrOfRLOFOpticalFlowParameter {
	#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::RLOFOpticalFlowParameterTrait for PtrOfRLOFOpticalFlowParameter {
	#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

