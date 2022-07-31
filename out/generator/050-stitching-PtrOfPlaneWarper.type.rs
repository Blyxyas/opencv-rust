pub type PtrOfPlaneWarper = core::Ptr<crate::stitching::PlaneWarper>;

ptr_extern! { crate::stitching::PlaneWarper,
	cv_PtrOfPlaneWarper_delete, cv_PtrOfPlaneWarper_get_inner_ptr, cv_PtrOfPlaneWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::PlaneWarper, cv_PtrOfPlaneWarper_new }

impl PtrOfPlaneWarper {
	#[inline] pub fn as_raw_PtrOfPlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::PlaneWarperTraitConst for PtrOfPlaneWarper {
	#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PlaneWarperTrait for PtrOfPlaneWarper {
	#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfPlaneWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfPlaneWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfPlaneWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfPlaneWarper_to_PtrOfWarperCreator,
}

