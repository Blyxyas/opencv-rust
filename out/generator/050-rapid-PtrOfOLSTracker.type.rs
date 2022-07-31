pub type PtrOfOLSTracker = core::Ptr<dyn crate::rapid::OLSTracker>;

ptr_extern! { dyn crate::rapid::OLSTracker,
	cv_PtrOfOLSTracker_delete, cv_PtrOfOLSTracker_get_inner_ptr, cv_PtrOfOLSTracker_get_inner_ptr_mut
}

impl PtrOfOLSTracker {
	#[inline] pub fn as_raw_PtrOfOLSTracker(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOLSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::OLSTrackerConst for PtrOfOLSTracker {
	#[inline] fn as_raw_OLSTracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::OLSTracker for PtrOfOLSTracker {
	#[inline] fn as_raw_mut_OLSTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfOLSTracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfOLSTracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rapid::TrackerConst for PtrOfOLSTracker {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Tracker for PtrOfOLSTracker {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

