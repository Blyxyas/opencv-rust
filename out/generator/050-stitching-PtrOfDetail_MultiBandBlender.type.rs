pub type PtrOfDetail_MultiBandBlender = core::Ptr<crate::stitching::Detail_MultiBandBlender>;

ptr_extern! { crate::stitching::Detail_MultiBandBlender,
	cv_PtrOfDetail_MultiBandBlender_delete, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_MultiBandBlender, cv_PtrOfDetail_MultiBandBlender_new }

impl PtrOfDetail_MultiBandBlender {
	#[inline] pub fn as_raw_PtrOfDetail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_MultiBandBlenderTraitConst for PtrOfDetail_MultiBandBlender {
	#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_MultiBandBlenderTrait for PtrOfDetail_MultiBandBlender {
	#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_MultiBandBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_MultiBandBlender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_MultiBandBlender, core::Ptr<crate::stitching::Detail_Blender>,
	cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender,
}

