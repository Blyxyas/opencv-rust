pub type PtrOfCompressedRectilinearPortraitWarper = core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper>;

ptr_extern! { crate::stitching::CompressedRectilinearPortraitWarper,
	cv_PtrOfCompressedRectilinearPortraitWarper_delete, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::CompressedRectilinearPortraitWarper, cv_PtrOfCompressedRectilinearPortraitWarper_new }

impl PtrOfCompressedRectilinearPortraitWarper {
	#[inline] pub fn as_raw_PtrOfCompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for PtrOfCompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CompressedRectilinearPortraitWarperTrait for PtrOfCompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfCompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfCompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfCompressedRectilinearPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator,
}

