pub type PtrOfPriorBoxLayer = core::Ptr<crate::dnn::PriorBoxLayer>;

ptr_extern! { crate::dnn::PriorBoxLayer,
	cv_PtrOfPriorBoxLayer_delete, cv_PtrOfPriorBoxLayer_get_inner_ptr, cv_PtrOfPriorBoxLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrOfPriorBoxLayer_new }

impl PtrOfPriorBoxLayer {
	#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PriorBoxLayerTraitConst for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PriorBoxLayerTrait for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPriorBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

