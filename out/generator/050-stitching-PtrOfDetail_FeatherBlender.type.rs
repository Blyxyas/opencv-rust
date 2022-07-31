pub type PtrOfDetail_FeatherBlender = core::Ptr<crate::stitching::Detail_FeatherBlender>;

ptr_extern! { crate::stitching::Detail_FeatherBlender,
	cv_PtrOfDetail_FeatherBlender_delete, cv_PtrOfDetail_FeatherBlender_get_inner_ptr, cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_FeatherBlender, cv_PtrOfDetail_FeatherBlender_new }

impl PtrOfDetail_FeatherBlender {
	#[inline] pub fn as_raw_PtrOfDetail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeatherBlenderTraitConst for PtrOfDetail_FeatherBlender {
	#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeatherBlenderTrait for PtrOfDetail_FeatherBlender {
	#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_FeatherBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_FeatherBlender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_FeatherBlender, core::Ptr<crate::stitching::Detail_Blender>,
	cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender,
}

