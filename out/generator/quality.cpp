#include "ocvrs_common.hpp"
#include <opencv2/quality.hpp>
#include "quality_types.hpp"

extern "C" {
	cv::Algorithm* cv_QualityBRISQUE_to_Algorithm(cv::quality::QualityBRISQUE* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_QualityBRISQUE_delete(cv::quality::QualityBRISQUE* instance) {
		delete instance;
	}
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitybrisque.hpp:33
	void cv_quality_QualityBRISQUE_compute_const__InputArrayR(cv::quality::QualityBRISQUE* instance, const cv::_InputArray* img, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// create(const cv::String &, const cv::String &) /usr/include/opencv2/quality/qualitybrisque.hpp:40
	void cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(const char* model_file_path, const char* range_file_path, Result<cv::Ptr<cv::quality::QualityBRISQUE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(std::string(model_file_path), std::string(range_file_path));
			Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityBRISQUE>*>))
	}
	
	// create(const cv::Ptr<cv::ml::SVM> &, const cv::Mat &) /usr/include/opencv2/quality/qualitybrisque.hpp:47
	void cv_quality_QualityBRISQUE_create_const_Ptr_SVM_R_const_MatR(const cv::Ptr<cv::ml::SVM>* model, const cv::Mat* range, Result<cv::Ptr<cv::quality::QualityBRISQUE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityBRISQUE> ret = cv::quality::QualityBRISQUE::create(*model, *range);
			Ok(new cv::Ptr<cv::quality::QualityBRISQUE>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityBRISQUE>*>))
	}
	
	// compute(cv::InputArray, const cv::String &, const cv::String &) /usr/include/opencv2/quality/qualitybrisque.hpp:56
	void cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(const cv::_InputArray* img, const char* model_file_path, const char* range_file_path, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityBRISQUE::compute(*img, std::string(model_file_path), std::string(range_file_path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// computeFeatures(cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitybrisque.hpp:63
	void cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_OutputArray* features, Result_void* ocvrs_return) {
		try {
			cv::quality::QualityBRISQUE::computeFeatures(*img, *features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitybase.hpp:35
	void cv_quality_QualityBase_compute_const__InputArrayR(cv::quality::QualityBase* instance, const cv::_InputArray* img, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// getQualityMap(cv::OutputArray) /usr/include/opencv2/quality/qualitybase.hpp:38
	void cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(const cv::quality::QualityBase* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->getQualityMap(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clear() /usr/include/opencv2/quality/qualitybase.hpp:46
	void cv_quality_QualityBase_clear(cv::quality::QualityBase* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/quality/qualitybase.hpp:49
	void cv_quality_QualityBase_empty_const(const cv::quality::QualityBase* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	cv::Algorithm* cv_QualityGMSD_to_Algorithm(cv::quality::QualityGMSD* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_QualityGMSD_delete(cv::quality::QualityGMSD* instance) {
		delete instance;
	}
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:28
	void cv_quality_QualityGMSD_compute_const__InputArrayR(cv::quality::QualityGMSD* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// empty() /usr/include/opencv2/quality/qualitygmsd.hpp:31
	void cv_quality_QualityGMSD_empty_const(const cv::quality::QualityGMSD* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// clear() /usr/include/opencv2/quality/qualitygmsd.hpp:34
	void cv_quality_QualityGMSD_clear(cv::quality::QualityGMSD* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::InputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:40
	void cv_quality_QualityGMSD_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualityGMSD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityGMSD> ret = cv::quality::QualityGMSD::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualityGMSD>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityGMSD>*>))
	}
	
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:49
	void cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityGMSD::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	cv::Algorithm* cv_QualityMSE_to_Algorithm(cv::quality::QualityMSE* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_QualityMSE_delete(cv::quality::QualityMSE* instance) {
		delete instance;
	}
	// compute(cv::InputArrayOfArrays) /usr/include/opencv2/quality/qualitymse.hpp:25
	void cv_quality_QualityMSE_compute_const__InputArrayR(cv::quality::QualityMSE* instance, const cv::_InputArray* cmpImgs, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmpImgs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// empty() /usr/include/opencv2/quality/qualitymse.hpp:28
	void cv_quality_QualityMSE_empty_const(const cv::quality::QualityMSE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// clear() /usr/include/opencv2/quality/qualitymse.hpp:31
	void cv_quality_QualityMSE_clear(cv::quality::QualityMSE* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::InputArray) /usr/include/opencv2/quality/qualitymse.hpp:37
	void cv_quality_QualityMSE_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualityMSE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityMSE> ret = cv::quality::QualityMSE::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualityMSE>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityMSE>*>))
	}
	
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitymse.hpp:46
	void cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityMSE::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	cv::Algorithm* cv_QualityPSNR_to_Algorithm(cv::quality::QualityPSNR* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_QualityPSNR_delete(cv::quality::QualityPSNR* instance) {
		delete instance;
	}
	// create(cv::InputArray, double) /usr/include/opencv2/quality/qualitypsnr.hpp:38
	void cv_quality_QualityPSNR_create_const__InputArrayR_double(const cv::_InputArray* ref, double maxPixelValue, Result<cv::Ptr<cv::quality::QualityPSNR>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualityPSNR> ret = cv::quality::QualityPSNR::create(*ref, maxPixelValue);
			Ok(new cv::Ptr<cv::quality::QualityPSNR>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualityPSNR>*>))
	}
	
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitypsnr.hpp:48
	void cv_quality_QualityPSNR_compute_const__InputArrayR(cv::quality::QualityPSNR* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// empty() /usr/include/opencv2/quality/qualitypsnr.hpp:59
	void cv_quality_QualityPSNR_empty_const(const cv::quality::QualityPSNR* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// clear() /usr/include/opencv2/quality/qualitypsnr.hpp:62
	void cv_quality_QualityPSNR_clear(cv::quality::QualityPSNR* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/quality/qualitypsnr.hpp:72
	void cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, double maxPixelValue, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualityPSNR::compute(*ref, *cmp, *qualityMap, maxPixelValue);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// getMaxPixelValue() /usr/include/opencv2/quality/qualitypsnr.hpp:81
	void cv_quality_QualityPSNR_getMaxPixelValue_const(const cv::quality::QualityPSNR* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPixelValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxPixelValue(double) /usr/include/opencv2/quality/qualitypsnr.hpp:87
	void cv_quality_QualityPSNR_setMaxPixelValue_double(cv::quality::QualityPSNR* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxPixelValue(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_QualitySSIM_to_Algorithm(cv::quality::QualitySSIM* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_QualitySSIM_delete(cv::quality::QualitySSIM* instance) {
		delete instance;
	}
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualityssim.hpp:27
	void cv_quality_QualitySSIM_compute_const__InputArrayR(cv::quality::QualitySSIM* instance, const cv::_InputArray* cmp, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = instance->compute(*cmp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// empty() /usr/include/opencv2/quality/qualityssim.hpp:30
	void cv_quality_QualitySSIM_empty_const(const cv::quality::QualitySSIM* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// clear() /usr/include/opencv2/quality/qualityssim.hpp:33
	void cv_quality_QualitySSIM_clear(cv::quality::QualitySSIM* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::InputArray) /usr/include/opencv2/quality/qualityssim.hpp:39
	void cv_quality_QualitySSIM_create_const__InputArrayR(const cv::_InputArray* ref, Result<cv::Ptr<cv::quality::QualitySSIM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::quality::QualitySSIM> ret = cv::quality::QualitySSIM::create(*ref);
			Ok(new cv::Ptr<cv::quality::QualitySSIM>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::quality::QualitySSIM>*>))
	}
	
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualityssim.hpp:48
	void cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* ref, const cv::_InputArray* cmp, const cv::_OutputArray* qualityMap, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::quality::QualitySSIM::compute(*ref, *cmp, *qualityMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
}
