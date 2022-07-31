pub type PtrOfBackendWrapper = core::Ptr<dyn crate::dnn::BackendWrapper>;

ptr_extern! { dyn crate::dnn::BackendWrapper,
	cv_PtrOfBackendWrapper_delete, cv_PtrOfBackendWrapper_get_inner_ptr, cv_PtrOfBackendWrapper_get_inner_ptr_mut
}

impl PtrOfBackendWrapper {
	#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BackendWrapperConst for PtrOfBackendWrapper {
	#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
	#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

