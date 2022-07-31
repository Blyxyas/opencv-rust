pub type PtrOfProposalLayer = core::Ptr<crate::dnn::ProposalLayer>;

ptr_extern! { crate::dnn::ProposalLayer,
	cv_PtrOfProposalLayer_delete, cv_PtrOfProposalLayer_get_inner_ptr, cv_PtrOfProposalLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrOfProposalLayer_new }

impl PtrOfProposalLayer {
	#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ProposalLayerTraitConst for PtrOfProposalLayer {
	#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ProposalLayerTrait for PtrOfProposalLayer {
	#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfProposalLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfProposalLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfProposalLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfProposalLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

