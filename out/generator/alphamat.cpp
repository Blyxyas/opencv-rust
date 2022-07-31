#include "alphamat.hpp"
#include "alphamat_types.hpp"

extern "C" {
	// infoFlow(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/alphamat.hpp:36
	void cv_alphamat_infoFlow_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_InputArray* tmap, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			cv::alphamat::infoFlow(*image, *tmap, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
