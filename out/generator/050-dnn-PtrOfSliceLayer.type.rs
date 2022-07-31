pub type PtrOfSliceLayer = core::Ptr<crate::dnn::SliceLayer>;

ptr_extern! { crate::dnn::SliceLayer,
	cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr, cv_PtrOfSliceLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrOfSliceLayer_new }

impl PtrOfSliceLayer {
	#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SliceLayerTraitConst for PtrOfSliceLayer {
	#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SliceLayerTrait for PtrOfSliceLayer {
	#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSliceLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSliceLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSliceLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSliceLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

