pub type PtrOfGridBoard = core::Ptr<crate::aruco::GridBoard>;

ptr_extern! { crate::aruco::GridBoard,
	cv_PtrOfGridBoard_delete, cv_PtrOfGridBoard_get_inner_ptr, cv_PtrOfGridBoard_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::GridBoard, cv_PtrOfGridBoard_new }

impl PtrOfGridBoard {
	#[inline] pub fn as_raw_PtrOfGridBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::GridBoardTraitConst for PtrOfGridBoard {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::GridBoardTrait for PtrOfGridBoard {
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for PtrOfGridBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for PtrOfGridBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

