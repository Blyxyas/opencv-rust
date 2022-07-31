pub type PtrOfCumSumLayer = core::Ptr<crate::dnn::CumSumLayer>;

ptr_extern! { crate::dnn::CumSumLayer,
	cv_PtrOfCumSumLayer_delete, cv_PtrOfCumSumLayer_get_inner_ptr, cv_PtrOfCumSumLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrOfCumSumLayer_new }

impl PtrOfCumSumLayer {
	#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CumSumLayerTraitConst for PtrOfCumSumLayer {
	#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CumSumLayerTrait for PtrOfCumSumLayer {
	#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCumSumLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCumSumLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCumSumLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCumSumLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

