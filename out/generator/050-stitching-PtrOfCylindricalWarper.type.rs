pub type PtrOfCylindricalWarper = core::Ptr<crate::stitching::CylindricalWarper>;

ptr_extern! { crate::stitching::CylindricalWarper,
	cv_PtrOfCylindricalWarper_delete, cv_PtrOfCylindricalWarper_get_inner_ptr, cv_PtrOfCylindricalWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::CylindricalWarper, cv_PtrOfCylindricalWarper_new }

impl PtrOfCylindricalWarper {
	#[inline] pub fn as_raw_PtrOfCylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::CylindricalWarperTraitConst for PtrOfCylindricalWarper {
	#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CylindricalWarperTrait for PtrOfCylindricalWarper {
	#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfCylindricalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfCylindricalWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfCylindricalWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator,
}

