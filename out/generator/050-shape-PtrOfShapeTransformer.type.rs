pub type PtrOfShapeTransformer = core::Ptr<dyn crate::shape::ShapeTransformer>;

ptr_extern! { dyn crate::shape::ShapeTransformer,
	cv_PtrOfShapeTransformer_delete, cv_PtrOfShapeTransformer_get_inner_ptr, cv_PtrOfShapeTransformer_get_inner_ptr_mut
}

impl PtrOfShapeTransformer {
	#[inline] pub fn as_raw_PtrOfShapeTransformer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::ShapeTransformerConst for PtrOfShapeTransformer {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformer for PtrOfShapeTransformer {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfShapeTransformer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfShapeTransformer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

