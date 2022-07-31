#include "ocvrs_common.hpp"
#include <opencv2/stereo.hpp>
#include "stereo_types.hpp"

extern "C" {
	// censusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:22
	void cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* image1, const cv::Mat* image2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, Result_void* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, *image2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// censusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:24
	void cv_stereo_censusTransform_const_MatR_int_MatR_const_int(const cv::Mat* image1, int kernelSize, cv::Mat* dist1, const int type, Result_void* ocvrs_return) {
		try {
			cv::stereo::censusTransform(*image1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// modifiedCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int, int, const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:29
	void cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, int t, const cv::Mat* integralImage1, const cv::Mat* integralImage2, Result_void* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type, t, *integralImage1, *integralImage2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// modifiedCensusTransform(const cv::Mat &, int, cv::Mat &, const int, int, const cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:31
	void cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, const int type, int t, const cv::Mat* integralImage, Result_void* ocvrs_return) {
		try {
			cv::stereo::modifiedCensusTransform(*img1, kernelSize, *dist, type, t, *integralImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// starCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:39
	void cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, Result_void* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// starCensusTransform(const cv::Mat &, int, cv::Mat &) /usr/include/opencv2/stereo/descriptor.hpp:41
	void cv_stereo_starCensusTransform_const_MatR_int_MatR(const cv::Mat* img1, int kernelSize, cv::Mat* dist, Result_void* ocvrs_return) {
		try {
			cv::stereo::starCensusTransform(*img1, kernelSize, *dist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// symetricCensusTransform(const cv::Mat &, const cv::Mat &, int, cv::Mat &, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:35
	void cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(const cv::Mat* img1, const cv::Mat* img2, int kernelSize, cv::Mat* dist1, cv::Mat* dist2, const int type, Result_void* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, *img2, kernelSize, *dist1, *dist2, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// symetricCensusTransform(const cv::Mat &, int, cv::Mat &, const int) /usr/include/opencv2/stereo/descriptor.hpp:37
	void cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(const cv::Mat* img1, int kernelSize, cv::Mat* dist1, const int type, Result_void* ocvrs_return) {
		try {
			cv::stereo::symetricCensusTransform(*img1, kernelSize, *dist1, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// MatchQuasiDense() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:33
	void cv_stereo_MatchQuasiDense_MatchQuasiDense(Result<cv::stereo::MatchQuasiDense>* ocvrs_return) {
		try {
			cv::stereo::MatchQuasiDense ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::stereo::MatchQuasiDense>))
	}
	
	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	void cv_stereo_QuasiDenseStereo_getPropParam_const(const cv::stereo::QuasiDenseStereo* instance, cv::stereo::PropagationParameters* ocvrs_return) {
			cv::stereo::PropagationParameters ret = instance->Param;
			*ocvrs_return = ret;
	}
	
	// Param /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:190
	void cv_stereo_QuasiDenseStereo_setPropParam_PropagationParameters(cv::stereo::QuasiDenseStereo* instance, cv::stereo::PropagationParameters* val) {
			instance->Param = *val;
	}
	
	// loadParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:118
	void cv_stereo_QuasiDenseStereo_loadParameters_String(cv::stereo::QuasiDenseStereo* instance, char* filepath, Result<int>* ocvrs_return) {
		try {
			int ret = instance->loadParameters(std::string(filepath));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// saveParameters(cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:129
	void cv_stereo_QuasiDenseStereo_saveParameters_String(cv::stereo::QuasiDenseStereo* instance, char* filepath, Result<int>* ocvrs_return) {
		try {
			int ret = instance->saveParameters(std::string(filepath));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getSparseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:138
	void cv_stereo_QuasiDenseStereo_getSparseMatches_vector_MatchQuasiDense_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* sMatches, Result_void* ocvrs_return) {
		try {
			instance->getSparseMatches(*sMatches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDenseMatches(std::vector<MatchQuasiDense> &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:147
	void cv_stereo_QuasiDenseStereo_getDenseMatches_vector_MatchQuasiDense_R(cv::stereo::QuasiDenseStereo* instance, std::vector<cv::stereo::MatchQuasiDense>* denseMatches, Result_void* ocvrs_return) {
		try {
			instance->getDenseMatches(*denseMatches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// process(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:163
	void cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(cv::stereo::QuasiDenseStereo* instance, const cv::Mat* imgLeft, const cv::Mat* imgRight, Result_void* ocvrs_return) {
		try {
			instance->process(*imgLeft, *imgRight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatch(const int, const int) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:174
	void cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(cv::stereo::QuasiDenseStereo* instance, const int x, const int y, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getMatch(x, y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// getDisparity() /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:184
	void cv_stereo_QuasiDenseStereo_getDisparity(cv::stereo::QuasiDenseStereo* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getDisparity();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// create(cv::Size, cv::String) /usr/include/opencv2/stereo/quasi_dense_stereo.hpp:187
	void cv_stereo_QuasiDenseStereo_create_Size_String(cv::Size* monoImgSize, char* paramFilepath, Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::stereo::QuasiDenseStereo> ret = cv::stereo::QuasiDenseStereo::create(*monoImgSize, std::string(paramFilepath));
			Ok(new cv::Ptr<cv::stereo::QuasiDenseStereo>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::stereo::QuasiDenseStereo>*>))
	}
	
}
