extern "C" {
	// getBackendName(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:27
	pub fn cv_videoio_registry_getBackendName_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<*mut c_void>);
	// getBackends() /usr/include/opencv2/videoio/registry.hpp:30
	pub fn cv_videoio_registry_getBackends(ocvrs_return: *mut Result<*mut c_void>);
	// getCameraBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:48
	pub fn cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
	// getCameraBackends() /usr/include/opencv2/videoio/registry.hpp:33
	pub fn cv_videoio_registry_getCameraBackends(ocvrs_return: *mut Result<*mut c_void>);
	// getStreamBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:55
	pub fn cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
	// getStreamBackends() /usr/include/opencv2/videoio/registry.hpp:36
	pub fn cv_videoio_registry_getStreamBackends(ocvrs_return: *mut Result<*mut c_void>);
	// getWriterBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:62
	pub fn cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(api: crate::videoio::VideoCaptureAPIs, version_abi: *mut i32, version_api: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
	// getWriterBackends() /usr/include/opencv2/videoio/registry.hpp:39
	pub fn cv_videoio_registry_getWriterBackends(ocvrs_return: *mut Result<*mut c_void>);
	// hasBackend(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:42
	pub fn cv_videoio_registry_hasBackend_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<bool>);
	// isBackendBuiltIn(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:45
	pub fn cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(api: crate::videoio::VideoCaptureAPIs, ocvrs_return: *mut Result<bool>);
	// VideoCapture() /usr/include/opencv2/videoio.hpp:690
	pub fn cv_VideoCapture_VideoCapture(ocvrs_return: *mut Result<*mut c_void>);
	// VideoCapture(const cv::String &, int) /usr/include/opencv2/videoio.hpp:707
	pub fn cv_VideoCapture_VideoCapture_const_StringR_int(filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
	// VideoCapture(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:715
	pub fn cv_VideoCapture_VideoCapture_const_StringR_int_const_vector_int_R(filename: *const c_char, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// VideoCapture(int, int) /usr/include/opencv2/videoio.hpp:727
	pub fn cv_VideoCapture_VideoCapture_int_int(index: i32, api_preference: i32, ocvrs_return: *mut Result<*mut c_void>);
	// VideoCapture(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:735
	pub fn cv_VideoCapture_VideoCapture_int_int_const_vector_int_R(index: i32, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// open(const cv::String &, int) /usr/include/opencv2/videoio.hpp:752
	pub fn cv_VideoCapture_open_const_StringR_int(instance: *mut c_void, filename: *const c_char, api_preference: i32, ocvrs_return: *mut Result<bool>);
	// open(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:765
	pub fn cv_VideoCapture_open_const_StringR_int_const_vector_int_R(instance: *mut c_void, filename: *const c_char, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// open(int, int) /usr/include/opencv2/videoio.hpp:776
	pub fn cv_VideoCapture_open_int_int(instance: *mut c_void, index: i32, api_preference: i32, ocvrs_return: *mut Result<bool>);
	// open(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:789
	pub fn cv_VideoCapture_open_int_int_const_vector_int_R(instance: *mut c_void, index: i32, api_preference: i32, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// isOpened() /usr/include/opencv2/videoio.hpp:796
	pub fn cv_VideoCapture_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// release() /usr/include/opencv2/videoio.hpp:805
	pub fn cv_VideoCapture_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// grab() /usr/include/opencv2/videoio.hpp:826
	pub fn cv_VideoCapture_grab(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// retrieve(cv::OutputArray, int) /usr/include/opencv2/videoio.hpp:844
	pub fn cv_VideoCapture_retrieve_const__OutputArrayR_int(instance: *mut c_void, image: *const c_void, flag: i32, ocvrs_return: *mut Result<bool>);
	// read(cv::OutputArray) /usr/include/opencv2/videoio.hpp:870
	pub fn cv_VideoCapture_read_const__OutputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
	// set(int, double) /usr/include/opencv2/videoio.hpp:881
	pub fn cv_VideoCapture_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
	// get(int) /usr/include/opencv2/videoio.hpp:900
	pub fn cv_VideoCapture_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
	// getBackendName() /usr/include/opencv2/videoio.hpp:906
	pub fn cv_VideoCapture_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setExceptionMode(bool) /usr/include/opencv2/videoio.hpp:912
	pub fn cv_VideoCapture_setExceptionMode_bool(instance: *mut c_void, enable: bool, ocvrs_return: *mut Result_void);
	// getExceptionMode() /usr/include/opencv2/videoio.hpp:915
	pub fn cv_VideoCapture_getExceptionMode(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// waitAny(const std::vector<VideoCapture> &, std::vector<int> &, int64) /usr/include/opencv2/videoio.hpp:933
	pub fn cv_VideoCapture_waitAny_const_vector_VideoCapture_R_vector_int_R_int64_t(streams: *const c_void, ready_index: *mut c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
	// VideoWriter() /usr/include/opencv2/videoio.hpp:970
	pub fn cv_VideoWriter_VideoWriter(ocvrs_return: *mut Result<*mut c_void>);
	// VideoWriter(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:993
	pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
	// VideoWriter(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1000
	pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<*mut c_void>);
	// VideoWriter(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1007
	pub fn cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vector_int_R(filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// VideoWriter(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1012
	pub fn cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vector_int_R(filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// open(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1029
	pub fn cv_VideoWriter_open_const_StringR_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
	// open(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1034
	pub fn cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, is_color: bool, ocvrs_return: *mut Result<bool>);
	// open(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1039
	pub fn cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vector_int_R(instance: *mut c_void, filename: *const c_char, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// open(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1044
	pub fn cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vector_int_R(instance: *mut c_void, filename: *const c_char, api_preference: i32, fourcc: i32, fps: f64, frame_size: *const core::Size, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// isOpened() /usr/include/opencv2/videoio.hpp:1049
	pub fn cv_VideoWriter_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// release() /usr/include/opencv2/videoio.hpp:1056
	pub fn cv_VideoWriter_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// write(cv::InputArray) /usr/include/opencv2/videoio.hpp:1075
	pub fn cv_VideoWriter_write_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// set(int, double) /usr/include/opencv2/videoio.hpp:1085
	pub fn cv_VideoWriter_set_int_double(instance: *mut c_void, prop_id: i32, value: f64, ocvrs_return: *mut Result<bool>);
	// get(int) /usr/include/opencv2/videoio.hpp:1095
	pub fn cv_VideoWriter_get_const_int(instance: *const c_void, prop_id: i32, ocvrs_return: *mut Result<f64>);
	// fourcc(char, char, char, char) /usr/include/opencv2/videoio.hpp:1104
	pub fn cv_VideoWriter_fourcc_char_char_char_char(c1: i8, c2: i8, c3: i8, c4: i8, ocvrs_return: *mut Result<i32>);
	// getBackendName() /usr/include/opencv2/videoio.hpp:1110
	pub fn cv_VideoWriter_getBackendName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
}
