pub type PtrOfMaxUnpoolLayer = core::Ptr<crate::dnn::MaxUnpoolLayer>;

ptr_extern! { crate::dnn::MaxUnpoolLayer,
	cv_PtrOfMaxUnpoolLayer_delete, cv_PtrOfMaxUnpoolLayer_get_inner_ptr, cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrOfMaxUnpoolLayer_new }

impl PtrOfMaxUnpoolLayer {
	#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MaxUnpoolLayerTraitConst for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MaxUnpoolLayerTrait for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfMaxUnpoolLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

