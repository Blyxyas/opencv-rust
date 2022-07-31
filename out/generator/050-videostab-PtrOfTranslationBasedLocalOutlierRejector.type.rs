pub type PtrOfTranslationBasedLocalOutlierRejector = core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector>;

ptr_extern! { crate::videostab::TranslationBasedLocalOutlierRejector,
	cv_PtrOfTranslationBasedLocalOutlierRejector_delete, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::TranslationBasedLocalOutlierRejector, cv_PtrOfTranslationBasedLocalOutlierRejector_new }

impl PtrOfTranslationBasedLocalOutlierRejector {
	#[inline] pub fn as_raw_PtrOfTranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst for PtrOfTranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for PtrOfTranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorConst for PtrOfTranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejector for PtrOfTranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfTranslationBasedLocalOutlierRejector, core::Ptr<dyn crate::videostab::IOutlierRejector>,
	cv_PtrOfTranslationBasedLocalOutlierRejector_to_PtrOfIOutlierRejector,
}

