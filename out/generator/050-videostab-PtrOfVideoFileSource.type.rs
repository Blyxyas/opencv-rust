pub type PtrOfVideoFileSource = core::Ptr<crate::videostab::VideoFileSource>;

ptr_extern! { crate::videostab::VideoFileSource,
	cv_PtrOfVideoFileSource_delete, cv_PtrOfVideoFileSource_get_inner_ptr, cv_PtrOfVideoFileSource_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::VideoFileSource, cv_PtrOfVideoFileSource_new }

impl PtrOfVideoFileSource {
	#[inline] pub fn as_raw_PtrOfVideoFileSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVideoFileSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::VideoFileSourceTraitConst for PtrOfVideoFileSource {
	#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::VideoFileSourceTrait for PtrOfVideoFileSource {
	#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfVideoFileSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfVideoFileSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfVideoFileSource, core::Ptr<dyn crate::videostab::IFrameSource>,
	cv_PtrOfVideoFileSource_to_PtrOfIFrameSource,
}

