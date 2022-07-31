pub type PtrOfBinaryDescriptor = core::Ptr<crate::line_descriptor::BinaryDescriptor>;

ptr_extern! { crate::line_descriptor::BinaryDescriptor,
	cv_PtrOfBinaryDescriptor_delete, cv_PtrOfBinaryDescriptor_get_inner_ptr, cv_PtrOfBinaryDescriptor_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptor, cv_PtrOfBinaryDescriptor_new }

impl PtrOfBinaryDescriptor {
	#[inline] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorTraitConst for PtrOfBinaryDescriptor {
	#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorTrait for PtrOfBinaryDescriptor {
	#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBinaryDescriptor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBinaryDescriptor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

