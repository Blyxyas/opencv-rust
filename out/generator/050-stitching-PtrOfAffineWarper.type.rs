pub type PtrOfAffineWarper = core::Ptr<crate::stitching::AffineWarper>;

ptr_extern! { crate::stitching::AffineWarper,
	cv_PtrOfAffineWarper_delete, cv_PtrOfAffineWarper_get_inner_ptr, cv_PtrOfAffineWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::AffineWarper, cv_PtrOfAffineWarper_new }

impl PtrOfAffineWarper {
	#[inline] pub fn as_raw_PtrOfAffineWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::AffineWarperTraitConst for PtrOfAffineWarper {
	#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::AffineWarperTrait for PtrOfAffineWarper {
	#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfAffineWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfAffineWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAffineWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfAffineWarper_to_PtrOfWarperCreator,
}

