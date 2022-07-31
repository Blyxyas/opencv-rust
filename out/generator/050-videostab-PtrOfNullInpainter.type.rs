pub type PtrOfNullInpainter = core::Ptr<crate::videostab::NullInpainter>;

ptr_extern! { crate::videostab::NullInpainter,
	cv_PtrOfNullInpainter_delete, cv_PtrOfNullInpainter_get_inner_ptr, cv_PtrOfNullInpainter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullInpainter, cv_PtrOfNullInpainter_new }

impl PtrOfNullInpainter {
	#[inline] pub fn as_raw_PtrOfNullInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullInpainterTraitConst for PtrOfNullInpainter {
	#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullInpainterTrait for PtrOfNullInpainter {
	#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfNullInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfNullInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfNullInpainter_to_PtrOfInpainterBase,
}

