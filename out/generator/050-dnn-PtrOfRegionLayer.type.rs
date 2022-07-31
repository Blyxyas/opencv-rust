pub type PtrOfRegionLayer = core::Ptr<crate::dnn::RegionLayer>;

ptr_extern! { crate::dnn::RegionLayer,
	cv_PtrOfRegionLayer_delete, cv_PtrOfRegionLayer_get_inner_ptr, cv_PtrOfRegionLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrOfRegionLayer_new }

impl PtrOfRegionLayer {
	#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RegionLayerTraitConst for PtrOfRegionLayer {
	#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RegionLayerTrait for PtrOfRegionLayer {
	#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRegionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRegionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfRegionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfRegionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

