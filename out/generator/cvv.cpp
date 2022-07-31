#include "ocvrs_common.hpp"
#include <opencv2/cvv.hpp>
#include "cvv_types.hpp"

extern "C" {
	// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const cvv::impl::CallMetaData &, const char *, const char *, bool) /usr/include/opencv2/cvv/dmatch.hpp:24
	void cvv_impl_debugDMatch_const__InputArrayR_vector_KeyPoint__const__InputArrayR_vector_KeyPoint__vector_DMatch__const_CallMetaDataR_const_charX_const_charX_bool(const cv::_InputArray* img1, std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, std::vector<cv::KeyPoint>* keypoints2, std::vector<cv::DMatch>* matches, const cvv::impl::CallMetaData* data, const char* description, const char* view, bool useTrainDescriptor, Result_void* ocvrs_return) {
		try {
			cvv::impl::debugDMatch(*img1, *keypoints1, *img2, *keypoints2, *matches, *data, description, view, useTrainDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// debugFilter(cv::InputArray, cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/filter.hpp:24
	void cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* original, const cv::_InputArray* result, const cvv::impl::CallMetaData* data, const char* description, const char* view, Result_void* ocvrs_return) {
		try {
			cvv::impl::debugFilter(*original, *result, *data, description, view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// finalShow() /usr/include/opencv2/cvv/final_show.hpp:15
	void cvv_impl_finalShow(Result_void* ocvrs_return) {
		try {
			cvv::impl::finalShow();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// showImage(cv::InputArray, const cvv::impl::CallMetaData &, const char *, const char *) /usr/include/opencv2/cvv/show_image.hpp:24
	void cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* img, const cvv::impl::CallMetaData* data, const char* description, const char* view, Result_void* ocvrs_return) {
		try {
			cvv::impl::showImage(*img, *data, description, view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// file /usr/include/opencv2/cvv/call_meta_data.hpp:46
	void* cvv_impl_CallMetaData_getPropFile_const(const cvv::impl::CallMetaData* instance) {
			const char* ret = instance->file;
			return ocvrs_create_string(ret);
	}
	
	// line /usr/include/opencv2/cvv/call_meta_data.hpp:47
	const size_t cvv_impl_CallMetaData_getPropLine_const(const cvv::impl::CallMetaData* instance) {
			const size_t ret = instance->line;
			return ret;
	}
	
	// function /usr/include/opencv2/cvv/call_meta_data.hpp:48
	void* cvv_impl_CallMetaData_getPropFunction_const(const cvv::impl::CallMetaData* instance) {
			const char* ret = instance->function;
			return ocvrs_create_string(ret);
	}
	
	// isKnown /usr/include/opencv2/cvv/call_meta_data.hpp:53
	const bool cvv_impl_CallMetaData_getPropIsKnown_const(const cvv::impl::CallMetaData* instance) {
			const bool ret = instance->isKnown;
			return ret;
	}
	
	void cv_CallMetaData_delete(cvv::impl::CallMetaData* instance) {
		delete instance;
	}
	// CallMetaData() /usr/include/opencv2/cvv/call_meta_data.hpp:26
	void cvv_impl_CallMetaData_CallMetaData(Result<cvv::impl::CallMetaData*>* ocvrs_return) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvv::impl::CallMetaData*>))
	}
	
	// CallMetaData(const char *, size_t, const char *) /usr/include/opencv2/cvv/call_meta_data.hpp:36
	void cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(const char* file, size_t line, const char* function, Result<cvv::impl::CallMetaData*>* ocvrs_return) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData(file, line, function);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvv::impl::CallMetaData*>))
	}
	
	// operator bool() /usr/include/opencv2/cvv/call_meta_data.hpp:40
	void cvv_impl_CallMetaData_operator_bool(cvv::impl::CallMetaData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator bool();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
}
