pub type PtrOfSinusoidalPattern_Params = core::Ptr<crate::structured_light::SinusoidalPattern_Params>;

ptr_extern! { crate::structured_light::SinusoidalPattern_Params,
	cv_PtrOfSinusoidalPattern_Params_delete, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::structured_light::SinusoidalPattern_Params, cv_PtrOfSinusoidalPattern_Params_new }

impl PtrOfSinusoidalPattern_Params {
	#[inline] pub fn as_raw_PtrOfSinusoidalPattern_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for PtrOfSinusoidalPattern_Params {
	#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::SinusoidalPattern_ParamsTrait for PtrOfSinusoidalPattern_Params {
	#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

