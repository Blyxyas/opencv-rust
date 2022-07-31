pub type PtrOfDictionary = core::Ptr<crate::aruco::Dictionary>;

ptr_extern! { crate::aruco::Dictionary,
	cv_PtrOfDictionary_delete, cv_PtrOfDictionary_get_inner_ptr, cv_PtrOfDictionary_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::Dictionary, cv_PtrOfDictionary_new }

impl PtrOfDictionary {
	#[inline] pub fn as_raw_PtrOfDictionary(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::DictionaryTraitConst for PtrOfDictionary {
	#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::DictionaryTrait for PtrOfDictionary {
	#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

