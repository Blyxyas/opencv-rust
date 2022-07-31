pub type PtrOfPaniniWarper = core::Ptr<crate::stitching::PaniniWarper>;

ptr_extern! { crate::stitching::PaniniWarper,
	cv_PtrOfPaniniWarper_delete, cv_PtrOfPaniniWarper_get_inner_ptr, cv_PtrOfPaniniWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::PaniniWarper, cv_PtrOfPaniniWarper_new }

impl PtrOfPaniniWarper {
	#[inline] pub fn as_raw_PtrOfPaniniWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::PaniniWarperTraitConst for PtrOfPaniniWarper {
	#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PaniniWarperTrait for PtrOfPaniniWarper {
	#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfPaniniWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfPaniniWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfPaniniWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfPaniniWarper_to_PtrOfWarperCreator,
}

