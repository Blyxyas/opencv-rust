pub type PtrOfSphericalWarper = core::Ptr<crate::stitching::SphericalWarper>;

ptr_extern! { crate::stitching::SphericalWarper,
	cv_PtrOfSphericalWarper_delete, cv_PtrOfSphericalWarper_get_inner_ptr, cv_PtrOfSphericalWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::SphericalWarper, cv_PtrOfSphericalWarper_new }

impl PtrOfSphericalWarper {
	#[inline] pub fn as_raw_PtrOfSphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::SphericalWarperTraitConst for PtrOfSphericalWarper {
	#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::SphericalWarperTrait for PtrOfSphericalWarper {
	#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfSphericalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfSphericalWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSphericalWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfSphericalWarper_to_PtrOfWarperCreator,
}

