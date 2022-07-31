pub type PtrOfCompressedRectilinearWarper = core::Ptr<crate::stitching::CompressedRectilinearWarper>;

ptr_extern! { crate::stitching::CompressedRectilinearWarper,
	cv_PtrOfCompressedRectilinearWarper_delete, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::CompressedRectilinearWarper, cv_PtrOfCompressedRectilinearWarper_new }

impl PtrOfCompressedRectilinearWarper {
	#[inline] pub fn as_raw_PtrOfCompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::CompressedRectilinearWarperTraitConst for PtrOfCompressedRectilinearWarper {
	#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CompressedRectilinearWarperTrait for PtrOfCompressedRectilinearWarper {
	#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfCompressedRectilinearWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfCompressedRectilinearWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfCompressedRectilinearWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator,
}

