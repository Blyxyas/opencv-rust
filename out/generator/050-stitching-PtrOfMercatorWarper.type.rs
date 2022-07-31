pub type PtrOfMercatorWarper = core::Ptr<crate::stitching::MercatorWarper>;

ptr_extern! { crate::stitching::MercatorWarper,
	cv_PtrOfMercatorWarper_delete, cv_PtrOfMercatorWarper_get_inner_ptr, cv_PtrOfMercatorWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::MercatorWarper, cv_PtrOfMercatorWarper_new }

impl PtrOfMercatorWarper {
	#[inline] pub fn as_raw_PtrOfMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::MercatorWarperTraitConst for PtrOfMercatorWarper {
	#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::MercatorWarperTrait for PtrOfMercatorWarper {
	#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfMercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfMercatorWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfMercatorWarper_to_PtrOfWarperCreator,
}

