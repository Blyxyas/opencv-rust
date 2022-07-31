pub type PtrOfStereographicWarper = core::Ptr<crate::stitching::StereographicWarper>;

ptr_extern! { crate::stitching::StereographicWarper,
	cv_PtrOfStereographicWarper_delete, cv_PtrOfStereographicWarper_get_inner_ptr, cv_PtrOfStereographicWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::StereographicWarper, cv_PtrOfStereographicWarper_new }

impl PtrOfStereographicWarper {
	#[inline] pub fn as_raw_PtrOfStereographicWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::StereographicWarperTraitConst for PtrOfStereographicWarper {
	#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::StereographicWarperTrait for PtrOfStereographicWarper {
	#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfStereographicWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfStereographicWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStereographicWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfStereographicWarper_to_PtrOfWarperCreator,
}

