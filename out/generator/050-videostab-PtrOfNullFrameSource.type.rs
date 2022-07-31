pub type PtrOfNullFrameSource = core::Ptr<crate::videostab::NullFrameSource>;

ptr_extern! { crate::videostab::NullFrameSource,
	cv_PtrOfNullFrameSource_delete, cv_PtrOfNullFrameSource_get_inner_ptr, cv_PtrOfNullFrameSource_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullFrameSource, cv_PtrOfNullFrameSource_new }

impl PtrOfNullFrameSource {
	#[inline] pub fn as_raw_PtrOfNullFrameSource(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullFrameSourceTraitConst for PtrOfNullFrameSource {
	#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullFrameSourceTrait for PtrOfNullFrameSource {
	#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfNullFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfNullFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullFrameSource, core::Ptr<dyn crate::videostab::IFrameSource>,
	cv_PtrOfNullFrameSource_to_PtrOfIFrameSource,
}

