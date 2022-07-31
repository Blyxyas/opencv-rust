pub type PtrOfDnnSuperResImpl = core::Ptr<crate::dnn_superres::DnnSuperResImpl>;

ptr_extern! { crate::dnn_superres::DnnSuperResImpl,
	cv_PtrOfDnnSuperResImpl_delete, cv_PtrOfDnnSuperResImpl_get_inner_ptr, cv_PtrOfDnnSuperResImpl_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn_superres::DnnSuperResImpl, cv_PtrOfDnnSuperResImpl_new }

impl PtrOfDnnSuperResImpl {
	#[inline] pub fn as_raw_PtrOfDnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn_superres::DnnSuperResImplTraitConst for PtrOfDnnSuperResImpl {
	#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn_superres::DnnSuperResImplTrait for PtrOfDnnSuperResImpl {
	#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

