pub type PtrOfKinfu_Params = core::Ptr<crate::rgbd::Kinfu_Params>;

ptr_extern! { crate::rgbd::Kinfu_Params,
	cv_PtrOfKinfu_Params_delete, cv_PtrOfKinfu_Params_get_inner_ptr, cv_PtrOfKinfu_Params_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Kinfu_Params, cv_PtrOfKinfu_Params_new }

impl PtrOfKinfu_Params {
	#[inline] pub fn as_raw_PtrOfKinfu_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_ParamsTraitConst for PtrOfKinfu_Params {
	#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_ParamsTrait for PtrOfKinfu_Params {
	#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

