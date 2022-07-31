pub type PtrOfFlowWarpLayer = core::Ptr<crate::dnn::FlowWarpLayer>;

ptr_extern! { crate::dnn::FlowWarpLayer,
	cv_PtrOfFlowWarpLayer_delete, cv_PtrOfFlowWarpLayer_get_inner_ptr, cv_PtrOfFlowWarpLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrOfFlowWarpLayer_new }

impl PtrOfFlowWarpLayer {
	#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FlowWarpLayerTraitConst for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::FlowWarpLayerTrait for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfFlowWarpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

