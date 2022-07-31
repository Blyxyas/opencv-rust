pub type PtrOfReorgLayer = core::Ptr<crate::dnn::ReorgLayer>;

ptr_extern! { crate::dnn::ReorgLayer,
	cv_PtrOfReorgLayer_delete, cv_PtrOfReorgLayer_get_inner_ptr, cv_PtrOfReorgLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrOfReorgLayer_new }

impl PtrOfReorgLayer {
	#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReorgLayerTraitConst for PtrOfReorgLayer {
	#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReorgLayerTrait for PtrOfReorgLayer {
	#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReorgLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReorgLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReorgLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReorgLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

