pub type PtrOfFlattenLayer = core::Ptr<crate::dnn::FlattenLayer>;

ptr_extern! { crate::dnn::FlattenLayer,
	cv_PtrOfFlattenLayer_delete, cv_PtrOfFlattenLayer_get_inner_ptr, cv_PtrOfFlattenLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrOfFlattenLayer_new }

impl PtrOfFlattenLayer {
	#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FlattenLayerTraitConst for PtrOfFlattenLayer {
	#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::FlattenLayerTrait for PtrOfFlattenLayer {
	#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFlattenLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFlattenLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfFlattenLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfFlattenLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

