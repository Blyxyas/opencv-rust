pub type PtrOfThinPlateSplineShapeTransformer = core::Ptr<dyn crate::shape::ThinPlateSplineShapeTransformer>;

ptr_extern! { dyn crate::shape::ThinPlateSplineShapeTransformer,
	cv_PtrOfThinPlateSplineShapeTransformer_delete, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut
}

impl PtrOfThinPlateSplineShapeTransformer {
	#[inline] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::shape::ThinPlateSplineShapeTransformerConst for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::shape::ShapeTransformerConst for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfThinPlateSplineShapeTransformer, core::Ptr<dyn crate::shape::ShapeTransformer>,
	cv_PtrOfThinPlateSplineShapeTransformer_to_PtrOfShapeTransformer,
}

