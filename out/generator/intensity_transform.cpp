#include "ocvrs_common.hpp"
#include <opencv2/intensity_transform.hpp>
#include "intensity_transform_types.hpp"

extern "C" {
	// BIMEF(cv::InputArray, cv::OutputArray, float, float, float) /usr/include/opencv2/intensity_transform.hpp:88
	void cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* input, const cv::_OutputArray* output, float mu, float a, float b, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::BIMEF(*input, *output, mu, a, b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// BIMEF(cv::InputArray, cv::OutputArray, float, float, float, float) /usr/include/opencv2/intensity_transform.hpp:106
	void cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float_float(const cv::_InputArray* input, const cv::_OutputArray* output, float k, float mu, float a, float b, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::BIMEF(*input, *output, k, mu, a, b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// autoscaling(const cv::Mat, cv::Mat &) /usr/include/opencv2/intensity_transform.hpp:60
	void cv_intensity_transform_autoscaling_const_Mat_MatR(const cv::Mat* input, cv::Mat* output, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::autoscaling(*input, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// contrastStretching(const cv::Mat, cv::Mat &, const int, const int, const int, const int) /usr/include/opencv2/intensity_transform.hpp:73
	void cv_intensity_transform_contrastStretching_const_Mat_MatR_const_int_const_int_const_int_const_int(const cv::Mat* input, cv::Mat* output, const int r1, const int s1, const int r2, const int s2, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::contrastStretching(*input, *output, r1, s1, r2, s2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// gammaCorrection(const cv::Mat, cv::Mat &, const float) /usr/include/opencv2/intensity_transform.hpp:51
	void cv_intensity_transform_gammaCorrection_const_Mat_MatR_const_float(const cv::Mat* input, cv::Mat* output, const float gamma, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::gammaCorrection(*input, *output, gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// logTransform(const cv::Mat, cv::Mat &) /usr/include/opencv2/intensity_transform.hpp:41
	void cv_intensity_transform_logTransform_const_Mat_MatR(const cv::Mat* input, cv::Mat* output, Result_void* ocvrs_return) {
		try {
			cv::intensity_transform::logTransform(*input, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
