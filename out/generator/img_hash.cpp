#include "ocvrs_common.hpp"
#include <opencv2/img_hash.hpp>
#include "img_hash_types.hpp"

extern "C" {
	// averageHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/average_hash.hpp:33
	void cv_img_hash_averageHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, Result_void* ocvrs_return) {
		try {
			cv::img_hash::averageHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blockMeanHash(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:44
	void cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, int mode, Result_void* ocvrs_return) {
		try {
			cv::img_hash::blockMeanHash(*inputArr, *outputArr, mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// colorMomentHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/color_moment_hash.hpp:35
	void cv_img_hash_colorMomentHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, Result_void* ocvrs_return) {
		try {
			cv::img_hash::colorMomentHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// marrHildrethHash(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:56
	void cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR_float_float(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, float alpha, float scale, Result_void* ocvrs_return) {
		try {
			cv::img_hash::marrHildrethHash(*inputArr, *outputArr, alpha, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// pHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/phash.hpp:35
	void cv_img_hash_pHash_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, Result_void* ocvrs_return) {
		try {
			cv::img_hash::pHash(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radialVarianceHash(cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:48
	void cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, double sigma, int numOfAngleLine, Result_void* ocvrs_return) {
		try {
			cv::img_hash::radialVarianceHash(*inputArr, *outputArr, sigma, numOfAngleLine);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_AverageHash_to_Algorithm(cv::img_hash::AverageHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_AverageHash_to_ImgHashBase(cv::img_hash::AverageHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_AverageHash_delete(cv::img_hash::AverageHash* instance) {
		delete instance;
	}
	// create() /usr/include/opencv2/img_hash/average_hash.hpp:24
	void cv_img_hash_AverageHash_create(Result<cv::Ptr<cv::img_hash::AverageHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::AverageHash> ret = cv::img_hash::AverageHash::create();
			Ok(new cv::Ptr<cv::img_hash::AverageHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::AverageHash>*>))
	}
	
	cv::Algorithm* cv_BlockMeanHash_to_Algorithm(cv::img_hash::BlockMeanHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_BlockMeanHash_to_ImgHashBase(cv::img_hash::BlockMeanHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_BlockMeanHash_delete(cv::img_hash::BlockMeanHash* instance) {
		delete instance;
	}
	// setMode(int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:32
	void cv_img_hash_BlockMeanHash_setMode_int(cv::img_hash::BlockMeanHash* instance, int mode, Result_void* ocvrs_return) {
		try {
			instance->setMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMean() /usr/include/opencv2/img_hash/block_mean_hash.hpp:33
	void cv_img_hash_BlockMeanHash_getMean_const(const cv::img_hash::BlockMeanHash* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->getMean();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	// create(int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:34
	void cv_img_hash_BlockMeanHash_create_int(int mode, Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::BlockMeanHash> ret = cv::img_hash::BlockMeanHash::create(mode);
			Ok(new cv::Ptr<cv::img_hash::BlockMeanHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>))
	}
	
	cv::Algorithm* cv_ColorMomentHash_to_Algorithm(cv::img_hash::ColorMomentHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_ColorMomentHash_to_ImgHashBase(cv::img_hash::ColorMomentHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_ColorMomentHash_delete(cv::img_hash::ColorMomentHash* instance) {
		delete instance;
	}
	// create() /usr/include/opencv2/img_hash/color_moment_hash.hpp:23
	void cv_img_hash_ColorMomentHash_create(Result<cv::Ptr<cv::img_hash::ColorMomentHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::ColorMomentHash> ret = cv::img_hash::ColorMomentHash::create();
			Ok(new cv::Ptr<cv::img_hash::ColorMomentHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::ColorMomentHash>*>))
	}
	
	cv::Algorithm* cv_ImgHashBase_to_Algorithm(cv::img_hash::ImgHashBase* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ImgHashBase_delete(cv::img_hash::ImgHashBase* instance) {
		delete instance;
	}
	// compute(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/img_hash_base.hpp:28
	void cv_img_hash_ImgHashBase_compute_const__InputArrayR_const__OutputArrayR(cv::img_hash::ImgHashBase* instance, const cv::_InputArray* inputArr, const cv::_OutputArray* outputArr, Result_void* ocvrs_return) {
		try {
			instance->compute(*inputArr, *outputArr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compare(cv::InputArray, cv::InputArray) /usr/include/opencv2/img_hash/img_hash_base.hpp:35
	void cv_img_hash_ImgHashBase_compare_const_const__InputArrayR_const__InputArrayR(const cv::img_hash::ImgHashBase* instance, const cv::_InputArray* hashOne, const cv::_InputArray* hashTwo, Result<double>* ocvrs_return) {
		try {
			double ret = instance->compare(*hashOne, *hashTwo);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	cv::Algorithm* cv_MarrHildrethHash_to_Algorithm(cv::img_hash::MarrHildrethHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_MarrHildrethHash_to_ImgHashBase(cv::img_hash::MarrHildrethHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_MarrHildrethHash_delete(cv::img_hash::MarrHildrethHash* instance) {
		delete instance;
	}
	// getAlpha() /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:26
	void cv_img_hash_MarrHildrethHash_getAlpha_const(const cv::img_hash::MarrHildrethHash* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getScale() /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:31
	void cv_img_hash_MarrHildrethHash_getScale_const(const cv::img_hash::MarrHildrethHash* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setKernelParam(float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:37
	void cv_img_hash_MarrHildrethHash_setKernelParam_float_float(cv::img_hash::MarrHildrethHash* instance, float alpha, float scale, Result_void* ocvrs_return) {
		try {
			instance->setKernelParam(alpha, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:43
	void cv_img_hash_MarrHildrethHash_create_float_float(float alpha, float scale, Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::MarrHildrethHash> ret = cv::img_hash::MarrHildrethHash::create(alpha, scale);
			Ok(new cv::Ptr<cv::img_hash::MarrHildrethHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>))
	}
	
	cv::Algorithm* cv_PHash_to_Algorithm(cv::img_hash::PHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_PHash_to_ImgHashBase(cv::img_hash::PHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_PHash_delete(cv::img_hash::PHash* instance) {
		delete instance;
	}
	// create() /usr/include/opencv2/img_hash/phash.hpp:25
	void cv_img_hash_PHash_create(Result<cv::Ptr<cv::img_hash::PHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::PHash> ret = cv::img_hash::PHash::create();
			Ok(new cv::Ptr<cv::img_hash::PHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::PHash>*>))
	}
	
	cv::Algorithm* cv_RadialVarianceHash_to_Algorithm(cv::img_hash::RadialVarianceHash* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::img_hash::ImgHashBase* cv_RadialVarianceHash_to_ImgHashBase(cv::img_hash::RadialVarianceHash* instance) {
		return dynamic_cast<cv::img_hash::ImgHashBase*>(instance);
	}
	
	void cv_RadialVarianceHash_delete(cv::img_hash::RadialVarianceHash* instance) {
		delete instance;
	}
	// create(double, int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:24
	void cv_img_hash_RadialVarianceHash_create_double_int(double sigma, int numOfAngleLine, Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::img_hash::RadialVarianceHash> ret = cv::img_hash::RadialVarianceHash::create(sigma, numOfAngleLine);
			Ok(new cv::Ptr<cv::img_hash::RadialVarianceHash>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>))
	}
	
	// getNumOfAngleLine() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:26
	void cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(const cv::img_hash::RadialVarianceHash* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumOfAngleLine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getSigma() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:27
	void cv_img_hash_RadialVarianceHash_getSigma_const(const cv::img_hash::RadialVarianceHash* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setNumOfAngleLine(int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:29
	void cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(cv::img_hash::RadialVarianceHash* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setNumOfAngleLine(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSigma(double) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:30
	void cv_img_hash_RadialVarianceHash_setSigma_double(cv::img_hash::RadialVarianceHash* instance, double value, Result_void* ocvrs_return) {
		try {
			instance->setSigma(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFeatures() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:33
	void cv_img_hash_RadialVarianceHash_getFeatures(cv::img_hash::RadialVarianceHash* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->getFeatures();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	// getHash() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:34
	void cv_img_hash_RadialVarianceHash_getHash(cv::img_hash::RadialVarianceHash* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getHash();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getPixPerLine(const cv::Mat &) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:35
	void cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatR(cv::img_hash::RadialVarianceHash* instance, const cv::Mat* input, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPixPerLine(*input);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getProjection() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:36
	void cv_img_hash_RadialVarianceHash_getProjection(cv::img_hash::RadialVarianceHash* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getProjection();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
}
