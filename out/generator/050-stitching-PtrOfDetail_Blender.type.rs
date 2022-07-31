pub type PtrOfDetail_Blender = core::Ptr<crate::stitching::Detail_Blender>;

ptr_extern! { crate::stitching::Detail_Blender,
	cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr, cv_PtrOfDetail_Blender_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrOfDetail_Blender_new }

impl PtrOfDetail_Blender {
	#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_Blender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_Blender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

