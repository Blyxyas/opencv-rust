pub type PtrOfIndexParams = core::Ptr<crate::flann::IndexParams>;

ptr_extern! { crate::flann::IndexParams,
	cv_PtrOfIndexParams_delete, cv_PtrOfIndexParams_get_inner_ptr, cv_PtrOfIndexParams_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::flann::IndexParams, cv_PtrOfIndexParams_new }

impl PtrOfIndexParams {
	#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for PtrOfIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for PtrOfIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

