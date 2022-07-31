pub type PtrOfRapid = core::Ptr<dyn crate::rapid::Rapid>;

ptr_extern! { dyn crate::rapid::Rapid,
	cv_PtrOfRapid_delete, cv_PtrOfRapid_get_inner_ptr, cv_PtrOfRapid_get_inner_ptr_mut
}

impl PtrOfRapid {
	#[inline] pub fn as_raw_PtrOfRapid(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRapid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::RapidConst for PtrOfRapid {
	#[inline] fn as_raw_Rapid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Rapid for PtrOfRapid {
	#[inline] fn as_raw_mut_Rapid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRapid {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRapid {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rapid::TrackerConst for PtrOfRapid {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Tracker for PtrOfRapid {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

