pub type PtrOfFormatted = core::Ptr<dyn core::Formatted>;

ptr_extern! { dyn core::Formatted,
	cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr, cv_PtrOfFormatted_get_inner_ptr_mut
}

impl PtrOfFormatted {
	#[inline] pub fn as_raw_PtrOfFormatted(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::FormattedConst for PtrOfFormatted {
	#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::Formatted for PtrOfFormatted {
	#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

