pub type PtrOfFisheyeWarper = core::Ptr<crate::stitching::FisheyeWarper>;

ptr_extern! { crate::stitching::FisheyeWarper,
	cv_PtrOfFisheyeWarper_delete, cv_PtrOfFisheyeWarper_get_inner_ptr, cv_PtrOfFisheyeWarper_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::FisheyeWarper, cv_PtrOfFisheyeWarper_new }

impl PtrOfFisheyeWarper {
	#[inline] pub fn as_raw_PtrOfFisheyeWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::FisheyeWarperTraitConst for PtrOfFisheyeWarper {
	#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::FisheyeWarperTrait for PtrOfFisheyeWarper {
	#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorConst for PtrOfFisheyeWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreator for PtrOfFisheyeWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfFisheyeWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
	cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator,
}

