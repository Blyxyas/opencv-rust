pub type PtrOfStandardCollector = core::Ptr<crate::face::StandardCollector>;

ptr_extern! { crate::face::StandardCollector,
	cv_PtrOfStandardCollector_delete, cv_PtrOfStandardCollector_get_inner_ptr, cv_PtrOfStandardCollector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::face::StandardCollector, cv_PtrOfStandardCollector_new }

impl PtrOfStandardCollector {
	#[inline] pub fn as_raw_PtrOfStandardCollector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::StandardCollectorTraitConst for PtrOfStandardCollector {
	#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::StandardCollectorTrait for PtrOfStandardCollector {
	#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::PredictCollectorConst for PtrOfStandardCollector {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::PredictCollector for PtrOfStandardCollector {
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStandardCollector, core::Ptr<dyn crate::face::PredictCollector>,
	cv_PtrOfStandardCollector_to_PtrOfPredictCollector,
}

