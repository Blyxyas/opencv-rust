pub type PtrOfAffineTransformer = core::Ptr<dyn crate::shape::AffineTransformer>;

ptr_extern! { dyn crate::shape::AffineTransformer,
	cv_PtrOfAffineTransformer_delete, cv_PtrOfAffineTransformer_get_inner_ptr, cv_PtrOfAffineTransformer_get_inner_ptr_mut
}

impl PtrOfAffineTransformer {
	#[inline] pub fn as_raw_PtrOfAffineTransformer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::AffineTransformerConst for PtrOfAffineTransformer {
	#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
	#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAffineTransformer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAffineTransformer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::ShapeTransformerConst for PtrOfAffineTransformer {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAffineTransformer, core::Ptr<dyn crate::shape::ShapeTransformer>,
	cv_PtrOfAffineTransformer_to_PtrOfShapeTransformer,
}

