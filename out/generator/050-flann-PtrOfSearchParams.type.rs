pub type PtrOfSearchParams = core::Ptr<crate::flann::SearchParams>;

ptr_extern! { crate::flann::SearchParams,
	cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr, cv_PtrOfSearchParams_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrOfSearchParams_new }

impl PtrOfSearchParams {
	#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::SearchParamsTraitConst for PtrOfSearchParams {
	#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::SearchParamsTrait for PtrOfSearchParams {
	#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for PtrOfSearchParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for PtrOfSearchParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

