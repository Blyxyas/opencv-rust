pub type PtrOfConcatLayer = core::Ptr<crate::dnn::ConcatLayer>;

ptr_extern! { crate::dnn::ConcatLayer,
	cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr, cv_PtrOfConcatLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrOfConcatLayer_new }

impl PtrOfConcatLayer {
	#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ConcatLayerTraitConst for PtrOfConcatLayer {
	#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ConcatLayerTrait for PtrOfConcatLayer {
	#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfConcatLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfConcatLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfConcatLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfConcatLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

