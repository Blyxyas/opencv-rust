pub type PtrOfOriginalClassName = core::Ptr<core::OriginalClassName>;

ptr_extern! { core::OriginalClassName,
	cv_PtrOfOriginalClassName_delete, cv_PtrOfOriginalClassName_get_inner_ptr, cv_PtrOfOriginalClassName_get_inner_ptr_mut
}

ptr_extern_ctor! { core::OriginalClassName, cv_PtrOfOriginalClassName_new }

impl PtrOfOriginalClassName {
	#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::OriginalClassNameTraitConst for PtrOfOriginalClassName {
	#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::OriginalClassNameTrait for PtrOfOriginalClassName {
	#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

