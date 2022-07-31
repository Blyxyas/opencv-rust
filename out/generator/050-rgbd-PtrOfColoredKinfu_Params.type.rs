pub type PtrOfColoredKinfu_Params = core::Ptr<crate::rgbd::ColoredKinfu_Params>;

ptr_extern! { crate::rgbd::ColoredKinfu_Params,
	cv_PtrOfColoredKinfu_Params_delete, cv_PtrOfColoredKinfu_Params_get_inner_ptr, cv_PtrOfColoredKinfu_Params_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::ColoredKinfu_Params, cv_PtrOfColoredKinfu_Params_new }

impl PtrOfColoredKinfu_Params {
	#[inline] pub fn as_raw_PtrOfColoredKinfu_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::ColoredKinfu_ParamsTraitConst for PtrOfColoredKinfu_Params {
	#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ColoredKinfu_ParamsTrait for PtrOfColoredKinfu_Params {
	#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

