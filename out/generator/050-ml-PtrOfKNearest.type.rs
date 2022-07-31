pub type PtrOfKNearest = core::Ptr<dyn crate::ml::KNearest>;

ptr_extern! { dyn crate::ml::KNearest,
	cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr, cv_PtrOfKNearest_get_inner_ptr_mut
}

impl PtrOfKNearest {
	#[inline] pub fn as_raw_PtrOfKNearest(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::KNearestConst for PtrOfKNearest {
	#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::KNearest for PtrOfKNearest {
	#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfKNearest {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfKNearest {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfKNearest {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfKNearest {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

