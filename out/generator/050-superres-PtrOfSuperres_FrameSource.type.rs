pub type PtrOfSuperres_FrameSource = core::Ptr<dyn crate::superres::Superres_FrameSource>;

ptr_extern! { dyn crate::superres::Superres_FrameSource,
	cv_PtrOfSuperres_FrameSource_delete, cv_PtrOfSuperres_FrameSource_get_inner_ptr, cv_PtrOfSuperres_FrameSource_get_inner_ptr_mut
}

impl PtrOfSuperres_FrameSource {
	#[inline] pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_FrameSourceConst for PtrOfSuperres_FrameSource {
	#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_FrameSource for PtrOfSuperres_FrameSource {
	#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

