#include "ocvrs_common.hpp"
#include <opencv2/fuzzy.hpp>
#include "fuzzy_types.hpp"

extern "C" {
	// FT02D_FL_process(cv::InputArray, const int, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:106
	void cv_ft_FT02D_FL_process_const__InputArrayR_const_int_const__OutputArrayR(const cv::_InputArray* matrix, const int radius, const cv::_OutputArray* output, Result_void* ocvrs_return) {
		try {
			cv::ft::FT02D_FL_process(*matrix, radius, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT02D_FL_process_float(cv::InputArray, const int, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:115
	void cv_ft_FT02D_FL_process_float_const__InputArrayR_const_int_const__OutputArrayR(const cv::_InputArray* matrix, const int radius, const cv::_OutputArray* output, Result_void* ocvrs_return) {
		try {
			cv::ft::FT02D_FL_process_float(*matrix, radius, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT02D_components(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
	void cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::ft::FT02D_components(*matrix, *kernel, *components, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT02D_inverseFT(cv::InputArray, cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:75
	void cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height, Result_void* ocvrs_return) {
		try {
			cv::ft::FT02D_inverseFT(*components, *kernel, *output, width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT02D_iteration(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:97
	void cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, const cv::_OutputArray* maskOutput, bool firstStop, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ft::FT02D_iteration(*matrix, *kernel, *output, *mask, *maskOutput, firstStop);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// FT02D_process(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
	void cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::ft::FT02D_process(*matrix, *kernel, *output, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_components(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:63
	void cv_ft_FT12D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* components, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_components(*matrix, *kernel, *components);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_createPolynomMatrixHorizontal(int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:94
	void cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayR_const_int(int radius, const cv::_OutputArray* matrix, const int chn, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_createPolynomMatrixHorizontal(radius, *matrix, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_createPolynomMatrixVertical(int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:85
	void cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayR_const_int(int radius, const cv::_OutputArray* matrix, const int chn, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_createPolynomMatrixVertical(radius, *matrix, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_inverseFT(cv::InputArray, cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:118
	void cv_ft_FT12D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* components, const cv::_InputArray* kernel, const cv::_OutputArray* output, int width, int height, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_inverseFT(*components, *kernel, *output, width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_polynomial(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
	void cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* c00, const cv::_OutputArray* c10, const cv::_OutputArray* c01, const cv::_OutputArray* components, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_polynomial(*matrix, *kernel, *c00, *c10, *c01, *components, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FT12D_process(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
	void cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* matrix, const cv::_InputArray* kernel, const cv::_OutputArray* output, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::ft::FT12D_process(*matrix, *kernel, *output, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createKernel(cv::InputArray, cv::InputArray, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:64
	void cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(const cv::_InputArray* A, const cv::_InputArray* B, const cv::_OutputArray* kernel, const int chn, Result_void* ocvrs_return) {
		try {
			cv::ft::createKernel(*A, *B, *kernel, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createKernel(int, int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:75
	void cv_ft_createKernel_int_int_const__OutputArrayR_const_int(int function, int radius, const cv::_OutputArray* kernel, const int chn, Result_void* ocvrs_return) {
		try {
			cv::ft::createKernel(function, radius, *kernel, chn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:103
	void cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* kernel, const cv::_OutputArray* output, Result_void* ocvrs_return) {
		try {
			cv::ft::filter(*image, *kernel, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// inpaint(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:94
	void cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* output, int radius, int function, int algorithm, Result_void* ocvrs_return) {
		try {
			cv::ft::inpaint(*image, *mask, *output, radius, function, algorithm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
