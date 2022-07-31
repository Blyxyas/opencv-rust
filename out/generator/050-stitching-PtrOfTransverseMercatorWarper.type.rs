pub type PtrOfTransverseMercatorWarper = core::Ptr<crate::stitching::TransverseMercatorWarper>;

ptr_extern! { crate::stitching::TransverseMercatorWarper,
	cv_PtrOfTransverseMercatorWarper_delete, cv_PtrOfTransverseMercatorWarper_get_inner_ptr, cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::TransverseMercatorWarper, cv_PtrOfTransverseMercatorWarper_new }

impl PtrOfTransverseMercatorWarper {
	#[inline] pub fn as_raw_PtrOfTransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::TransverseMercatorWarperTraitConst for PtrOfTransverseMercatorWarper {
	#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::TransverseMercatorWarperTrait for PtrOfTransverseMercatorWarper {
	#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfTransverseMercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfTransverseMercatorWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfTransverseMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator,
}

