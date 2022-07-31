pub type PtrOfParams = core::Ptr<crate::rgbd::Params>;

ptr_extern! { crate::rgbd::Params,
	cv_PtrOfParams_delete, cv_PtrOfParams_get_inner_ptr, cv_PtrOfParams_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Params, cv_PtrOfParams_new }

impl PtrOfParams {
	#[inline] pub fn as_raw_PtrOfParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::ParamsTraitConst for PtrOfParams {
	#[inline] fn as_raw_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ParamsTrait for PtrOfParams {
	#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

