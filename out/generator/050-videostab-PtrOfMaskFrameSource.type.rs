pub type PtrOfMaskFrameSource = core::Ptr<crate::videostab::MaskFrameSource>;

ptr_extern! { crate::videostab::MaskFrameSource,
	cv_PtrOfMaskFrameSource_delete, cv_PtrOfMaskFrameSource_get_inner_ptr, cv_PtrOfMaskFrameSource_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::MaskFrameSource, cv_PtrOfMaskFrameSource_new }

impl PtrOfMaskFrameSource {
	#[inline] pub fn as_raw_PtrOfMaskFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMaskFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MaskFrameSourceTraitConst for PtrOfMaskFrameSource {
	#[inline] fn as_raw_MaskFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MaskFrameSourceTrait for PtrOfMaskFrameSource {
	#[inline] fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfMaskFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfMaskFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMaskFrameSource, core::Ptr<dyn crate::videostab::IFrameSource>,
	cv_PtrOfMaskFrameSource_to_PtrOfIFrameSource,
}

