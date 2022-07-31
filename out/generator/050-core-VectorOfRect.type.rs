pub type VectorOfRect = core::Vector<core::Rect>;

impl VectorOfRect {
	pub fn as_raw_VectorOfRect(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfRect(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Rect, *const c_void, *mut c_void,
	cv_VectorOfRect_new, cv_VectorOfRect_delete,
	cv_VectorOfRect_len, cv_VectorOfRect_is_empty,
	cv_VectorOfRect_capacity, cv_VectorOfRect_resize,
	cv_VectorOfRect_shrink_to_fit,
	cv_VectorOfRect_reserve, cv_VectorOfRect_remove,
	cv_VectorOfRect_swap, cv_VectorOfRect_clear,
	cv_VectorOfRect_get, cv_VectorOfRect_set,
	cv_VectorOfRect_push, cv_VectorOfRect_insert,
}
vector_copy_non_bool! { core::Rect, *const c_void, *mut c_void,
	cv_VectorOfRect_data, cv_VectorOfRect_data_mut, cv_VectorOfRect_from_slice,
	cv_VectorOfRect_clone,
}

unsafe impl Send for core::Vector<core::Rect> {}

impl core::ToInputArray for VectorOfRect {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfRect_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfRect {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfRect_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfRect {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfRect_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfRect }

