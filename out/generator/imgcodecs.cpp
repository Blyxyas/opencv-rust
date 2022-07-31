#include "ocvrs_common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	// haveImageReader(const cv::String &) /usr/include/opencv2/imgcodecs.hpp:317
	void cv_haveImageReader_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageReader(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// haveImageWriter(const cv::String &) /usr/include/opencv2/imgcodecs.hpp:323
	void cv_haveImageWriter_const_StringR(const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveImageWriter(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imcount(const cv::String &, int) /usr/include/opencv2/imgcodecs.hpp:236
	void cv_imcount_const_StringR_int(const char* filename, int flags, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::imcount(std::string(filename), flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// imdecode(cv::InputArray, int) /usr/include/opencv2/imgcodecs.hpp:289
	void cv_imdecode_const__InputArrayR_int(const cv::_InputArray* buf, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// imdecode(cv::InputArray, int, cv::Mat *) /usr/include/opencv2/imgcodecs.hpp:297
	void cv_imdecode_const__InputArrayR_int_MatX(const cv::_InputArray* buf, int flags, cv::Mat* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags, dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// imencode(const cv::String &, cv::InputArray, std::vector<uchar> &, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:309
	void cv_imencode_const_StringR_const__InputArrayR_vector_unsigned_char_R_const_vector_int_R(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imread(const cv::String &, int) /usr/include/opencv2/imgcodecs.hpp:206
	void cv_imread_const_StringR_int(const char* filename, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(std::string(filename), flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// imreadmulti(const cv::String &, std::vector<Mat> &, int) /usr/include/opencv2/imgcodecs.hpp:216
	void cv_imreadmulti_const_StringR_vector_Mat_R_int(const char* filename, std::vector<cv::Mat>* mats, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imreadmulti(const cv::String &, std::vector<Mat> &, int, int, int) /usr/include/opencv2/imgcodecs.hpp:228
	void cv_imreadmulti_const_StringR_vector_Mat_R_int_int_int(const char* filename, std::vector<cv::Mat>* mats, int start, int count, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, start, count, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imwrite(const cv::String &, cv::InputArray, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:267
	void cv_imwrite_const_StringR_const__InputArrayR_const_vector_int_R(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imwritemulti(const cv::String &, cv::InputArrayOfArrays, const std::vector<int> &) /usr/include/opencv2/imgcodecs.hpp:272
	void cv_imwritemulti_const_StringR_const__InputArrayR_const_vector_int_R(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(std::string(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
}
