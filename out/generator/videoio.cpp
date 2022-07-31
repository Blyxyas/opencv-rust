#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	// getBackendName(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:27
	void cv_videoio_registry_getBackendName_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::videoio_registry::getBackendName(api);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getBackends() /usr/include/opencv2/videoio/registry.hpp:30
	void cv_videoio_registry_getBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	// getCameraBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:48
	void cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getCameraBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getCameraBackends() /usr/include/opencv2/videoio/registry.hpp:33
	void cv_videoio_registry_getCameraBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getCameraBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	// getStreamBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:55
	void cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getStreamBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getStreamBackends() /usr/include/opencv2/videoio/registry.hpp:36
	void cv_videoio_registry_getStreamBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	// getWriterBackendPluginVersion(cv::VideoCaptureAPIs, int &, int &) /usr/include/opencv2/videoio/registry.hpp:62
	void cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getWriterBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getWriterBackends() /usr/include/opencv2/videoio/registry.hpp:39
	void cv_videoio_registry_getWriterBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getWriterBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	// hasBackend(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:42
	void cv_videoio_registry_hasBackend_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::videoio_registry::hasBackend(api);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isBackendBuiltIn(cv::VideoCaptureAPIs) /usr/include/opencv2/videoio/registry.hpp:45
	void cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::videoio_registry::isBackendBuiltIn(api);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
		delete instance;
	}
	// VideoCapture() /usr/include/opencv2/videoio.hpp:690
	void cv_VideoCapture_VideoCapture(Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	// VideoCapture(const cv::String &, int) /usr/include/opencv2/videoio.hpp:707
	void cv_VideoCapture_VideoCapture_const_StringR_int(const char* filename, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	// VideoCapture(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:715
	void cv_VideoCapture_VideoCapture_const_StringR_int_const_vector_int_R(const char* filename, int apiPreference, const std::vector<int>* params, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	// VideoCapture(int, int) /usr/include/opencv2/videoio.hpp:727
	void cv_VideoCapture_VideoCapture_int_int(int index, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	// VideoCapture(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:735
	void cv_VideoCapture_VideoCapture_int_int_const_vector_int_R(int index, int apiPreference, const std::vector<int>* params, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	// open(const cv::String &, int) /usr/include/opencv2/videoio.hpp:752
	void cv_VideoCapture_open_const_StringR_int(cv::VideoCapture* instance, const char* filename, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(const cv::String &, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:765
	void cv_VideoCapture_open_const_StringR_int_const_vector_int_R(cv::VideoCapture* instance, const char* filename, int apiPreference, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(int, int) /usr/include/opencv2/videoio.hpp:776
	void cv_VideoCapture_open_int_int(cv::VideoCapture* instance, int index, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(int, int, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:789
	void cv_VideoCapture_open_int_int_const_vector_int_R(cv::VideoCapture* instance, int index, int apiPreference, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isOpened() /usr/include/opencv2/videoio.hpp:796
	void cv_VideoCapture_isOpened_const(const cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// release() /usr/include/opencv2/videoio.hpp:805
	void cv_VideoCapture_release(cv::VideoCapture* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// grab() /usr/include/opencv2/videoio.hpp:826
	void cv_VideoCapture_grab(cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->grab();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// retrieve(cv::OutputArray, int) /usr/include/opencv2/videoio.hpp:844
	void cv_VideoCapture_retrieve_const__OutputArrayR_int(cv::VideoCapture* instance, const cv::_OutputArray* image, int flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*image, flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// read(cv::OutputArray) /usr/include/opencv2/videoio.hpp:870
	void cv_VideoCapture_read_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// set(int, double) /usr/include/opencv2/videoio.hpp:881
	void cv_VideoCapture_set_int_double(cv::VideoCapture* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// get(int) /usr/include/opencv2/videoio.hpp:900
	void cv_VideoCapture_get_const_int(const cv::VideoCapture* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getBackendName() /usr/include/opencv2/videoio.hpp:906
	void cv_VideoCapture_getBackendName_const(const cv::VideoCapture* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getBackendName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// setExceptionMode(bool) /usr/include/opencv2/videoio.hpp:912
	void cv_VideoCapture_setExceptionMode_bool(cv::VideoCapture* instance, bool enable, Result_void* ocvrs_return) {
		try {
			instance->setExceptionMode(enable);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getExceptionMode() /usr/include/opencv2/videoio.hpp:915
	void cv_VideoCapture_getExceptionMode(cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExceptionMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// waitAny(const std::vector<VideoCapture> &, std::vector<int> &, int64) /usr/include/opencv2/videoio.hpp:933
	void cv_VideoCapture_waitAny_const_vector_VideoCapture_R_vector_int_R_int64_t(const std::vector<cv::VideoCapture>* streams, std::vector<int>* readyIndex, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::VideoCapture::waitAny(*streams, *readyIndex, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_VideoWriter_delete(cv::VideoWriter* instance) {
		delete instance;
	}
	// VideoWriter() /usr/include/opencv2/videoio.hpp:970
	void cv_VideoWriter_VideoWriter(Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	// VideoWriter(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:993
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	// VideoWriter(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1000
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	// VideoWriter(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1007
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vector_int_R(const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	// VideoWriter(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1012
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vector_int_R(const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	// open(const cv::String &, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1029
	void cv_VideoWriter_open_const_StringR_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(const cv::String &, int, int, double, cv::Size, bool) /usr/include/opencv2/videoio.hpp:1034
	void cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(const cv::String &, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1039
	void cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vector_int_R(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// open(const cv::String &, int, int, double, const cv::Size &, const std::vector<int> &) /usr/include/opencv2/videoio.hpp:1044
	void cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vector_int_R(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isOpened() /usr/include/opencv2/videoio.hpp:1049
	void cv_VideoWriter_isOpened_const(const cv::VideoWriter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// release() /usr/include/opencv2/videoio.hpp:1056
	void cv_VideoWriter_release(cv::VideoWriter* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::InputArray) /usr/include/opencv2/videoio.hpp:1075
	void cv_VideoWriter_write_const__InputArrayR(cv::VideoWriter* instance, const cv::_InputArray* image, Result_void* ocvrs_return) {
		try {
			instance->write(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// set(int, double) /usr/include/opencv2/videoio.hpp:1085
	void cv_VideoWriter_set_int_double(cv::VideoWriter* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// get(int) /usr/include/opencv2/videoio.hpp:1095
	void cv_VideoWriter_get_const_int(const cv::VideoWriter* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// fourcc(char, char, char, char) /usr/include/opencv2/videoio.hpp:1104
	void cv_VideoWriter_fourcc_char_char_char_char(char c1, char c2, char c3, char c4, Result<int>* ocvrs_return) {
		try {
			int ret = cv::VideoWriter::fourcc(c1, c2, c3, c4);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getBackendName() /usr/include/opencv2/videoio.hpp:1110
	void cv_VideoWriter_getBackendName_const(const cv::VideoWriter* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getBackendName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
}
