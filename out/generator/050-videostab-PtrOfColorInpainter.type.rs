pub type PtrOfColorInpainter = core::Ptr<crate::videostab::ColorInpainter>;

ptr_extern! { crate::videostab::ColorInpainter,
	cv_PtrOfColorInpainter_delete, cv_PtrOfColorInpainter_get_inner_ptr, cv_PtrOfColorInpainter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::ColorInpainter, cv_PtrOfColorInpainter_new }

impl PtrOfColorInpainter {
	#[inline] pub fn as_raw_PtrOfColorInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ColorInpainterTraitConst for PtrOfColorInpainter {
	#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ColorInpainterTrait for PtrOfColorInpainter {
	#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfColorInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfColorInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfColorInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfColorInpainter_to_PtrOfInpainterBase,
}

