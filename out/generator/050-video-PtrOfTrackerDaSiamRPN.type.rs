pub type PtrOfTrackerDaSiamRPN = core::Ptr<dyn crate::video::TrackerDaSiamRPN>;

ptr_extern! { dyn crate::video::TrackerDaSiamRPN,
	cv_PtrOfTrackerDaSiamRPN_delete, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr_mut
}

impl PtrOfTrackerDaSiamRPN {
	#[inline] pub fn as_raw_PtrOfTrackerDaSiamRPN(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerDaSiamRPN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::TrackerDaSiamRPNConst for PtrOfTrackerDaSiamRPN {
	#[inline] fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerDaSiamRPN for PtrOfTrackerDaSiamRPN {
	#[inline] fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerConst for PtrOfTrackerDaSiamRPN {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::Tracker for PtrOfTrackerDaSiamRPN {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

