pub type PtrOfWarperCreator = core::Ptr<dyn crate::stitching::WarperCreator>;

ptr_extern! { dyn crate::stitching::WarperCreator,
	cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr, cv_PtrOfWarperCreator_get_inner_ptr_mut
}

impl PtrOfWarperCreator {
	#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfWarperCreator {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfWarperCreator {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

