pub type PtrOfIFrameSource = core::Ptr<dyn crate::videostab::IFrameSource>;

ptr_extern! { dyn crate::videostab::IFrameSource,
	cv_PtrOfIFrameSource_delete, cv_PtrOfIFrameSource_get_inner_ptr, cv_PtrOfIFrameSource_get_inner_ptr_mut
}

impl PtrOfIFrameSource {
	#[inline] pub fn as_raw_PtrOfIFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfIFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfIFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

