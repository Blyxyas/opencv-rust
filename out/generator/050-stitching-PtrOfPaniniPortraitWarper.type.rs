pub type PtrOfPaniniPortraitWarper = core::Ptr<crate::stitching::PaniniPortraitWarper>;

ptr_extern! { crate::stitching::PaniniPortraitWarper,
	cv_PtrOfPaniniPortraitWarper_delete, cv_PtrOfPaniniPortraitWarper_get_inner_ptr, cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::PaniniPortraitWarper, cv_PtrOfPaniniPortraitWarper_new }

impl PtrOfPaniniPortraitWarper {
	#[inline] pub fn as_raw_PtrOfPaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::PaniniPortraitWarperTraitConst for PtrOfPaniniPortraitWarper {
	#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PaniniPortraitWarperTrait for PtrOfPaniniPortraitWarper {
	#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfPaniniPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfPaniniPortraitWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfPaniniPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator,
}

