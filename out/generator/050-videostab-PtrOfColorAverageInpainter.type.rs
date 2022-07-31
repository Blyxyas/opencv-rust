pub type PtrOfColorAverageInpainter = core::Ptr<crate::videostab::ColorAverageInpainter>;

ptr_extern! { crate::videostab::ColorAverageInpainter,
	cv_PtrOfColorAverageInpainter_delete, cv_PtrOfColorAverageInpainter_get_inner_ptr, cv_PtrOfColorAverageInpainter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::ColorAverageInpainter, cv_PtrOfColorAverageInpainter_new }

impl PtrOfColorAverageInpainter {
	#[inline] pub fn as_raw_PtrOfColorAverageInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorAverageInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ColorAverageInpainterTraitConst for PtrOfColorAverageInpainter {
	#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ColorAverageInpainterTrait for PtrOfColorAverageInpainter {
	#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfColorAverageInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfColorAverageInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfColorAverageInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfColorAverageInpainter_to_PtrOfInpainterBase,
}

