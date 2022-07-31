#include "ocvrs_common.hpp"
#include <opencv2/videostab.hpp>
#include "videostab_types.hpp"

extern "C" {
	// calcBlurriness(const cv::Mat &) /usr/include/opencv2/videostab/deblurring.hpp:57
	void cv_videostab_calcBlurriness_const_MatR(const cv::Mat* frame, Result<float>* ocvrs_return) {
		try {
			float ret = cv::videostab::calcBlurriness(*frame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// calcFlowMask(const cv::Mat &, const cv::Mat &, const cv::Mat &, float, const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:199
	void cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* errors, float maxError, const cv::Mat* mask0, const cv::Mat* mask1, cv::Mat* flowMask, Result_void* ocvrs_return) {
		try {
			cv::videostab::calcFlowMask(*flowX, *flowY, *errors, maxError, *mask0, *mask1, *flowMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// completeFrameAccordingToFlow(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, float, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:203
	void cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(const cv::Mat* flowMask, const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* frame1, const cv::Mat* mask1, float distThresh, cv::Mat* frame0, cv::Mat* mask0, Result_void* ocvrs_return) {
		try {
			cv::videostab::completeFrameAccordingToFlow(*flowMask, *flowX, *flowY, *frame1, *mask1, distThresh, *frame0, *mask0);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ensureInclusionConstraint(const cv::Mat &, cv::Size, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:165
	void cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(const cv::Mat* M, cv::Size* size, float trimRatio, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::ensureInclusionConstraint(*M, *size, trimRatio);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// estimateGlobalMotionLeastSquares(cv::InputOutputArray, cv::InputOutputArray, int, float *) /usr/include/opencv2/videostab/global_motion.hpp:77
	void cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(const cv::_InputOutputArray* points0, const cv::_InputOutputArray* points1, int model, float* rmse, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionLeastSquares(*points0, *points1, model, rmse);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// estimateGlobalMotionRansac(cv::InputArray, cv::InputArray, int, const cv::videostab::RansacParams &, float *, int *) /usr/include/opencv2/videostab/global_motion.hpp:90
	void cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(const cv::_InputArray* points0, const cv::_InputArray* points1, int model, const cv::videostab::RansacParams* params, float* rmse, int* ninliers, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionRansac(*points0, *points1, model, *params, rmse, ninliers);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// estimateOptimalTrimRatio(const cv::Mat &, cv::Size) /usr/include/opencv2/videostab/motion_stabilizing.hpp:167
	void cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(const cv::Mat* M, cv::Size* size, Result<float>* ocvrs_return) {
		try {
			float ret = cv::videostab::estimateOptimalTrimRatio(*M, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getMotion(int, int, const std::vector<Mat> &) /usr/include/opencv2/videostab/global_motion.hpp:304
	void cv_videostab_getMotion_int_int_const_vector_Mat_R(int from, int to, const std::vector<cv::Mat>* motions, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::getMotion(from, to, *motions);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_ColorAverageInpainter_delete(cv::videostab::ColorAverageInpainter* instance) {
		delete instance;
	}
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:177
	void cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(cv::videostab::ColorAverageInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ColorInpainter_delete(cv::videostab::ColorInpainter* instance) {
		delete instance;
	}
	// ColorInpainter(int, double) /usr/include/opencv2/videostab/inpainting.hpp:186
	void cv_videostab_ColorInpainter_ColorInpainter_int_double(int method, double radius, Result<cv::videostab::ColorInpainter*>* ocvrs_return) {
		try {
			cv::videostab::ColorInpainter* ret = new cv::videostab::ColorInpainter(method, radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::ColorInpainter*>))
	}
	
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:188
	void cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(cv::videostab::ColorInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ConsistentMosaicInpainter_delete(cv::videostab::ConsistentMosaicInpainter* instance) {
		delete instance;
	}
	// ConsistentMosaicInpainter() /usr/include/opencv2/videostab/inpainting.hpp:130
	void cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter(Result<cv::videostab::ConsistentMosaicInpainter*>* ocvrs_return) {
		try {
			cv::videostab::ConsistentMosaicInpainter* ret = new cv::videostab::ConsistentMosaicInpainter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::ConsistentMosaicInpainter*>))
	}
	
	// setStdevThresh(float) /usr/include/opencv2/videostab/inpainting.hpp:132
	void cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(cv::videostab::ConsistentMosaicInpainter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setStdevThresh(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stdevThresh() /usr/include/opencv2/videostab/inpainting.hpp:133
	void cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(const cv::videostab::ConsistentMosaicInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->stdevThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:135
	void cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(cv::videostab::ConsistentMosaicInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setRadius(int) /usr/include/opencv2/videostab/deblurring.hpp:66
	void cv_videostab_DeblurerBase_setRadius_int(cv::videostab::DeblurerBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radius() /usr/include/opencv2/videostab/deblurring.hpp:67
	void cv_videostab_DeblurerBase_radius_const(const cv::videostab::DeblurerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:69
	void cv_videostab_DeblurerBase_deblur_int_MatR_const_RangeR(cv::videostab::DeblurerBase* instance, int idx, cv::Mat* frame, const cv::Range* range, Result_void* ocvrs_return) {
		try {
			instance->deblur(idx, *frame, *range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/deblurring.hpp:74
	void cv_videostab_DeblurerBase_setFrames_const_vector_Mat_R(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// frames() /usr/include/opencv2/videostab/deblurring.hpp:75
	void cv_videostab_DeblurerBase_frames_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->frames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/deblurring.hpp:77
	void cv_videostab_DeblurerBase_setMotions_const_vector_Mat_R(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motions() /usr/include/opencv2/videostab/deblurring.hpp:78
	void cv_videostab_DeblurerBase_motions_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setBlurrinessRates(const std::vector<float> &) /usr/include/opencv2/videostab/deblurring.hpp:80
	void cv_videostab_DeblurerBase_setBlurrinessRates_const_vector_float_R(cv::videostab::DeblurerBase* instance, const std::vector<float>* val, Result_void* ocvrs_return) {
		try {
			instance->setBlurrinessRates(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blurrinessRates() /usr/include/opencv2/videostab/deblurring.hpp:81
	void cv_videostab_DeblurerBase_blurrinessRates_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<float>*>* ocvrs_return) {
		try {
			const std::vector<float> ret = instance->blurrinessRates();
			Ok(new const std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	void cv_FastMarchingMethod_delete(cv::videostab::FastMarchingMethod* instance) {
		delete instance;
	}
	// FastMarchingMethod() /usr/include/opencv2/videostab/fast_marching.hpp:66
	void cv_videostab_FastMarchingMethod_FastMarchingMethod(Result<cv::videostab::FastMarchingMethod*>* ocvrs_return) {
		try {
			cv::videostab::FastMarchingMethod* ret = new cv::videostab::FastMarchingMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::FastMarchingMethod*>))
	}
	
	// distanceMap() /usr/include/opencv2/videostab/fast_marching.hpp:81
	void cv_videostab_FastMarchingMethod_distanceMap_const(const cv::videostab::FastMarchingMethod* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->distanceMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_FromFileMotionReader_delete(cv::videostab::FromFileMotionReader* instance) {
		delete instance;
	}
	// FromFileMotionReader(const cv::String &) /usr/include/opencv2/videostab/global_motion.hpp:201
	void cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(const char* path, Result<cv::videostab::FromFileMotionReader*>* ocvrs_return) {
		try {
			cv::videostab::FromFileMotionReader* ret = new cv::videostab::FromFileMotionReader(std::string(path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::FromFileMotionReader*>))
	}
	
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:203
	void cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(cv::videostab::FromFileMotionReader* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_GaussianMotionFilter_delete(cv::videostab::GaussianMotionFilter* instance) {
		delete instance;
	}
	// GaussianMotionFilter(int, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:100
	void cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(int radius, float stdev, Result<cv::videostab::GaussianMotionFilter*>* ocvrs_return) {
		try {
			cv::videostab::GaussianMotionFilter* ret = new cv::videostab::GaussianMotionFilter(radius, stdev);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::GaussianMotionFilter*>))
	}
	
	// setParams(int, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:102
	void cv_videostab_GaussianMotionFilter_setParams_int_float(cv::videostab::GaussianMotionFilter* instance, int radius, float stdev, Result_void* ocvrs_return) {
		try {
			instance->setParams(radius, stdev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radius() /usr/include/opencv2/videostab/motion_stabilizing.hpp:103
	void cv_videostab_GaussianMotionFilter_radius_const(const cv::videostab::GaussianMotionFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// stdev() /usr/include/opencv2/videostab/motion_stabilizing.hpp:104
	void cv_videostab_GaussianMotionFilter_stdev_const(const cv::videostab::GaussianMotionFilter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->stdev();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &) /usr/include/opencv2/videostab/motion_stabilizing.hpp:106
	void cv_videostab_GaussianMotionFilter_stabilize_int_const_vector_Mat_R_const_RangeR(cv::videostab::GaussianMotionFilter* instance, int idx, const std::vector<cv::Mat>* motions, const cv::Range* range, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->stabilize(idx, *motions, *range);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// run(cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:74
	void cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(cv::videostab::IDenseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputOutputArray* flowX, const cv::_InputOutputArray* flowY, const cv::_OutputArray* errors, Result_void* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *flowX, *flowY, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:61
	void cv_videostab_IFrameSource_reset(cv::videostab::IFrameSource* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:62
	void cv_videostab_IFrameSource_nextFrame(cv::videostab::IFrameSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:60
	void cv_videostab_ILog_print_const_charX(cv::videostab::ILog* instance, const char* format, Result_void* ocvrs_return) {
		try {
			instance->print(format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:65
	void cv_videostab_IMotionStabilizer_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(cv::videostab::IMotionStabilizer* instance, int size, const std::vector<cv::Mat>* motions, const cv::Range* range, cv::Mat* stabilizationMotions, Result_void* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:63
	void cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::IOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// run(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:65
	void cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::videostab::ISparseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors, Result_void* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:180
	void cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::ImageMotionEstimatorBase* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:181
	void cv_videostab_ImageMotionEstimatorBase_motionModel_const(const cv::videostab::ImageMotionEstimatorBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:183
	void cv_videostab_ImageMotionEstimatorBase_setFrameMask_const__InputArrayR(cv::videostab::ImageMotionEstimatorBase* instance, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->setFrameMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:189
	void cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(cv::videostab::ImageMotionEstimatorBase* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setRadius(int) /usr/include/opencv2/videostab/inpainting.hpp:70
	void cv_videostab_InpainterBase_setRadius_int(cv::videostab::InpainterBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radius() /usr/include/opencv2/videostab/inpainting.hpp:71
	void cv_videostab_InpainterBase_radius_const(const cv::videostab::InpainterBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/inpainting.hpp:73
	void cv_videostab_InpainterBase_setMotionModel_MotionModel(cv::videostab::InpainterBase* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/inpainting.hpp:74
	void cv_videostab_InpainterBase_motionModel_const(const cv::videostab::InpainterBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:76
	void cv_videostab_InpainterBase_inpaint_int_MatR_MatR(cv::videostab::InpainterBase* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:81
	void cv_videostab_InpainterBase_setFrames_const_vector_Mat_R(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// frames() /usr/include/opencv2/videostab/inpainting.hpp:82
	void cv_videostab_InpainterBase_frames_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->frames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:84
	void cv_videostab_InpainterBase_setMotions_const_vector_Mat_R(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motions() /usr/include/opencv2/videostab/inpainting.hpp:85
	void cv_videostab_InpainterBase_motions_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setStabilizedFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:87
	void cv_videostab_InpainterBase_setStabilizedFrames_const_vector_Mat_R(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setStabilizedFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stabilizedFrames() /usr/include/opencv2/videostab/inpainting.hpp:88
	void cv_videostab_InpainterBase_stabilizedFrames_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizedFrames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:90
	void cv_videostab_InpainterBase_setStabilizationMotions_const_vector_Mat_R(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stabilizationMotions() /usr/include/opencv2/videostab/inpainting.hpp:91
	void cv_videostab_InpainterBase_stabilizationMotions_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizationMotions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	void cv_InpaintingPipeline_delete(cv::videostab::InpaintingPipeline* instance) {
		delete instance;
	}
	// pushBack(Ptr<cv::videostab::InpainterBase>) /usr/include/opencv2/videostab/inpainting.hpp:111
	void cv_videostab_InpaintingPipeline_pushBack_Ptr_InpainterBase_(cv::videostab::InpaintingPipeline* instance, cv::Ptr<cv::videostab::InpainterBase>* inpainter, Result_void* ocvrs_return) {
		try {
			instance->pushBack(*inpainter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/videostab/inpainting.hpp:112
	void cv_videostab_InpaintingPipeline_empty_const(const cv::videostab::InpaintingPipeline* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setRadius(int) /usr/include/opencv2/videostab/inpainting.hpp:114
	void cv_videostab_InpaintingPipeline_setRadius_int(cv::videostab::InpaintingPipeline* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/inpainting.hpp:115
	void cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(cv::videostab::InpaintingPipeline* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:116
	void cv_videostab_InpaintingPipeline_setFrames_const_vector_Mat_R(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:117
	void cv_videostab_InpaintingPipeline_setMotions_const_vector_Mat_R(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setStabilizedFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:118
	void cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vector_Mat_R(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setStabilizedFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:119
	void cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vector_Mat_R(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:121
	void cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(cv::videostab::InpaintingPipeline* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_KeypointBasedMotionEstimator_delete(cv::videostab::KeypointBasedMotionEstimator* instance) {
		delete instance;
	}
	// KeypointBasedMotionEstimator(Ptr<cv::videostab::MotionEstimatorBase>) /usr/include/opencv2/videostab/global_motion.hpp:232
	void cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_Ptr_MotionEstimatorBase_(cv::Ptr<cv::videostab::MotionEstimatorBase>* estimator, Result<cv::videostab::KeypointBasedMotionEstimator*>* ocvrs_return) {
		try {
			cv::videostab::KeypointBasedMotionEstimator* ret = new cv::videostab::KeypointBasedMotionEstimator(*estimator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::KeypointBasedMotionEstimator*>))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:234
	void cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(cv::videostab::KeypointBasedMotionEstimator* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:235
	void cv_videostab_KeypointBasedMotionEstimator_motionModel_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// setDetector(Ptr<cv::FeatureDetector>) /usr/include/opencv2/videostab/global_motion.hpp:237
	void cv_videostab_KeypointBasedMotionEstimator_setDetector_Ptr_Feature2D_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::Feature2D>* val, Result_void* ocvrs_return) {
		try {
			instance->setDetector(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detector() /usr/include/opencv2/videostab/global_motion.hpp:238
	void cv_videostab_KeypointBasedMotionEstimator_detector_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->detector();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	// setOpticalFlowEstimator(Ptr<cv::videostab::ISparseOptFlowEstimator>) /usr/include/opencv2/videostab/global_motion.hpp:240
	void cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_Ptr_ISparseOptFlowEstimator_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* val, Result_void* ocvrs_return) {
		try {
			instance->setOpticalFlowEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// opticalFlowEstimator() /usr/include/opencv2/videostab/global_motion.hpp:241
	void cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ISparseOptFlowEstimator> ret = instance->opticalFlowEstimator();
			Ok(new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>))
	}
	
	// setOutlierRejector(Ptr<cv::videostab::IOutlierRejector>) /usr/include/opencv2/videostab/global_motion.hpp:243
	void cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_Ptr_IOutlierRejector_(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::IOutlierRejector>* val, Result_void* ocvrs_return) {
		try {
			instance->setOutlierRejector(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// outlierRejector() /usr/include/opencv2/videostab/global_motion.hpp:244
	void cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::videostab::IOutlierRejector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IOutlierRejector> ret = instance->outlierRejector();
			Ok(new cv::Ptr<cv::videostab::IOutlierRejector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::IOutlierRejector>*>))
	}
	
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:246
	void cv_videostab_KeypointBasedMotionEstimator_setFrameMask_const__InputArrayR(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->setFrameMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:248
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:249
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_LogToStdout_delete(cv::videostab::LogToStdout* instance) {
		delete instance;
	}
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:72
	void cv_videostab_LogToStdout_print_const_charX(cv::videostab::LogToStdout* instance, const char* format, Result_void* ocvrs_return) {
		try {
			instance->print(format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LpMotionStabilizer_delete(cv::videostab::LpMotionStabilizer* instance) {
		delete instance;
	}
	// LpMotionStabilizer(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_stabilizing.hpp:120
	void cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::LpMotionStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::LpMotionStabilizer* ret = new cv::videostab::LpMotionStabilizer(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::LpMotionStabilizer*>))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_stabilizing.hpp:122
	void cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(cv::videostab::LpMotionStabilizer* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/motion_stabilizing.hpp:123
	void cv_videostab_LpMotionStabilizer_motionModel_const(const cv::videostab::LpMotionStabilizer* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// setFrameSize(cv::Size) /usr/include/opencv2/videostab/motion_stabilizing.hpp:125
	void cv_videostab_LpMotionStabilizer_setFrameSize_Size(cv::videostab::LpMotionStabilizer* instance, cv::Size* val, Result_void* ocvrs_return) {
		try {
			instance->setFrameSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// frameSize() /usr/include/opencv2/videostab/motion_stabilizing.hpp:126
	void cv_videostab_LpMotionStabilizer_frameSize_const(const cv::videostab::LpMotionStabilizer* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->frameSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setTrimRatio(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:128
	void cv_videostab_LpMotionStabilizer_setTrimRatio_float(cv::videostab::LpMotionStabilizer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// trimRatio() /usr/include/opencv2/videostab/motion_stabilizing.hpp:129
	void cv_videostab_LpMotionStabilizer_trimRatio_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->trimRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setWeight1(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:131
	void cv_videostab_LpMotionStabilizer_setWeight1_float(cv::videostab::LpMotionStabilizer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setWeight1(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// weight1() /usr/include/opencv2/videostab/motion_stabilizing.hpp:132
	void cv_videostab_LpMotionStabilizer_weight1_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setWeight2(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:134
	void cv_videostab_LpMotionStabilizer_setWeight2_float(cv::videostab::LpMotionStabilizer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setWeight2(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// weight2() /usr/include/opencv2/videostab/motion_stabilizing.hpp:135
	void cv_videostab_LpMotionStabilizer_weight2_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setWeight3(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:137
	void cv_videostab_LpMotionStabilizer_setWeight3_float(cv::videostab::LpMotionStabilizer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setWeight3(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// weight3() /usr/include/opencv2/videostab/motion_stabilizing.hpp:138
	void cv_videostab_LpMotionStabilizer_weight3_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight3();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setWeight4(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:140
	void cv_videostab_LpMotionStabilizer_setWeight4_float(cv::videostab::LpMotionStabilizer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setWeight4(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// weight4() /usr/include/opencv2/videostab/motion_stabilizing.hpp:141
	void cv_videostab_LpMotionStabilizer_weight4_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight4();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:143
	void cv_videostab_LpMotionStabilizer_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(cv::videostab::LpMotionStabilizer* instance, int size, const std::vector<cv::Mat>* motions, const cv::Range* range, cv::Mat* stabilizationMotions, Result_void* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MaskFrameSource_delete(cv::videostab::MaskFrameSource* instance) {
		delete instance;
	}
	// MaskFrameSource(const Ptr<cv::videostab::IFrameSource> &) /usr/include/opencv2/videostab/frame_source.hpp:92
	void cv_videostab_MaskFrameSource_MaskFrameSource_const_Ptr_IFrameSource_R(const cv::Ptr<cv::videostab::IFrameSource>* source, Result<cv::videostab::MaskFrameSource*>* ocvrs_return) {
		try {
			cv::videostab::MaskFrameSource* ret = new cv::videostab::MaskFrameSource(*source);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MaskFrameSource*>))
	}
	
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:94
	void cv_videostab_MaskFrameSource_reset(cv::videostab::MaskFrameSource* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:95
	void cv_videostab_MaskFrameSource_nextFrame(cv::videostab::MaskFrameSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_MoreAccurateMotionWobbleSuppressor_delete(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
		delete instance;
	}
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:116
	void cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result, Result_void* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setPeriod(int) /usr/include/opencv2/videostab/wobble_suppression.hpp:104
	void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setPeriod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// period() /usr/include/opencv2/videostab/wobble_suppression.hpp:105
	void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->period();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:106
	void cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::MotionEstimatorBase* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:111
	void cv_videostab_MotionEstimatorBase_motionModel_const(const cv::videostab::MotionEstimatorBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:120
	void cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorBase* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_MotionEstimatorL1_delete(cv::videostab::MotionEstimatorL1* instance) {
		delete instance;
	}
	// MotionEstimatorL1(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:156
	void cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::MotionEstimatorL1*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorL1* ret = new cv::videostab::MotionEstimatorL1(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionEstimatorL1*>))
	}
	
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:158
	void cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorL1* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_MotionEstimatorRansacL2_delete(cv::videostab::MotionEstimatorRansacL2* instance) {
		delete instance;
	}
	// MotionEstimatorRansacL2(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:134
	void cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::MotionEstimatorRansacL2*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorRansacL2* ret = new cv::videostab::MotionEstimatorRansacL2(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionEstimatorRansacL2*>))
	}
	
	// setRansacParams(const cv::videostab::RansacParams &) /usr/include/opencv2/videostab/global_motion.hpp:136
	void cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(cv::videostab::MotionEstimatorRansacL2* instance, const cv::videostab::RansacParams* val, Result_void* ocvrs_return) {
		try {
			instance->setRansacParams(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ransacParams() /usr/include/opencv2/videostab/global_motion.hpp:137
	void cv_videostab_MotionEstimatorRansacL2_ransacParams_const(const cv::videostab::MotionEstimatorRansacL2* instance, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::RansacParams*>))
	}
	
	// setMinInlierRatio(float) /usr/include/opencv2/videostab/global_motion.hpp:139
	void cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(cv::videostab::MotionEstimatorRansacL2* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setMinInlierRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// minInlierRatio() /usr/include/opencv2/videostab/global_motion.hpp:140
	void cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(const cv::videostab::MotionEstimatorRansacL2* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->minInlierRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:142
	void cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorRansacL2* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &) /usr/include/opencv2/videostab/motion_stabilizing.hpp:89
	void cv_videostab_MotionFilterBase_stabilize_int_const_vector_Mat_R_const_RangeR(cv::videostab::MotionFilterBase* instance, int idx, const std::vector<cv::Mat>* motions, const cv::Range* range, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->stabilize(idx, *motions, *range);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:92
	void cv_videostab_MotionFilterBase_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(cv::videostab::MotionFilterBase* instance, int size, const std::vector<cv::Mat>* motions, const cv::Range* range, cv::Mat* stabilizationMotions, Result_void* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MotionInpainter_delete(cv::videostab::MotionInpainter* instance) {
		delete instance;
	}
	// MotionInpainter() /usr/include/opencv2/videostab/inpainting.hpp:144
	void cv_videostab_MotionInpainter_MotionInpainter(Result<cv::videostab::MotionInpainter*>* ocvrs_return) {
		try {
			cv::videostab::MotionInpainter* ret = new cv::videostab::MotionInpainter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionInpainter*>))
	}
	
	// setOptFlowEstimator(Ptr<cv::videostab::IDenseOptFlowEstimator>) /usr/include/opencv2/videostab/inpainting.hpp:146
	void cv_videostab_MotionInpainter_setOptFlowEstimator_Ptr_IDenseOptFlowEstimator_(cv::videostab::MotionInpainter* instance, cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* val, Result_void* ocvrs_return) {
		try {
			instance->setOptFlowEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// optFlowEstimator() /usr/include/opencv2/videostab/inpainting.hpp:147
	void cv_videostab_MotionInpainter_optFlowEstimator_const(const cv::videostab::MotionInpainter* instance, Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IDenseOptFlowEstimator> ret = instance->optFlowEstimator();
			Ok(new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>))
	}
	
	// setFlowErrorThreshold(float) /usr/include/opencv2/videostab/inpainting.hpp:149
	void cv_videostab_MotionInpainter_setFlowErrorThreshold_float(cv::videostab::MotionInpainter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setFlowErrorThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flowErrorThreshold() /usr/include/opencv2/videostab/inpainting.hpp:150
	void cv_videostab_MotionInpainter_flowErrorThreshold_const(const cv::videostab::MotionInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->flowErrorThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setDistThreshold(float) /usr/include/opencv2/videostab/inpainting.hpp:152
	void cv_videostab_MotionInpainter_setDistThreshold_float(cv::videostab::MotionInpainter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setDistThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// distThresh() /usr/include/opencv2/videostab/inpainting.hpp:153
	void cv_videostab_MotionInpainter_distThresh_const(const cv::videostab::MotionInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->distThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setBorderMode(int) /usr/include/opencv2/videostab/inpainting.hpp:155
	void cv_videostab_MotionInpainter_setBorderMode_int(cv::videostab::MotionInpainter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setBorderMode(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// borderMode() /usr/include/opencv2/videostab/inpainting.hpp:156
	void cv_videostab_MotionInpainter_borderMode_const(const cv::videostab::MotionInpainter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->borderMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:158
	void cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(cv::videostab::MotionInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MotionStabilizationPipeline_delete(cv::videostab::MotionStabilizationPipeline* instance) {
		delete instance;
	}
	// pushBack(Ptr<cv::videostab::IMotionStabilizer>) /usr/include/opencv2/videostab/motion_stabilizing.hpp:73
	void cv_videostab_MotionStabilizationPipeline_pushBack_Ptr_IMotionStabilizer_(cv::videostab::MotionStabilizationPipeline* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* stabilizer, Result_void* ocvrs_return) {
		try {
			instance->pushBack(*stabilizer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/videostab/motion_stabilizing.hpp:74
	void cv_videostab_MotionStabilizationPipeline_empty_const(const cv::videostab::MotionStabilizationPipeline* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:76
	void cv_videostab_MotionStabilizationPipeline_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(cv::videostab::MotionStabilizationPipeline* instance, int size, const std::vector<cv::Mat>* motions, const cv::Range* range, cv::Mat* stabilizationMotions, Result_void* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NullDeblurer_delete(cv::videostab::NullDeblurer* instance) {
		delete instance;
	}
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:93
	void cv_videostab_NullDeblurer_deblur_int_MatR_const_RangeR(cv::videostab::NullDeblurer* instance, int unnamed, cv::Mat* unnamed_1, const cv::Range* unnamed_2, Result_void* ocvrs_return) {
		try {
			instance->deblur(unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NullFrameSource_delete(cv::videostab::NullFrameSource* instance) {
		delete instance;
	}
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:68
	void cv_videostab_NullFrameSource_reset(cv::videostab::NullFrameSource* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:69
	void cv_videostab_NullFrameSource_nextFrame(cv::videostab::NullFrameSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_NullInpainter_delete(cv::videostab::NullInpainter* instance) {
		delete instance;
	}
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:105
	void cv_videostab_NullInpainter_inpaint_int_MatR_MatR(cv::videostab::NullInpainter* instance, int unnamed, cv::Mat* unnamed_1, cv::Mat* unnamed_2, Result_void* ocvrs_return) {
		try {
			instance->inpaint(unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NullLog_delete(cv::videostab::NullLog* instance) {
		delete instance;
	}
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:66
	void cv_videostab_NullLog_print_const_charX(cv::videostab::NullLog* instance, const char* unnamed, Result_void* ocvrs_return) {
		try {
			instance->print(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NullOutlierRejector_delete(cv::videostab::NullOutlierRejector* instance) {
		delete instance;
	}
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:70
	void cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::NullOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_NullWobbleSuppressor_delete(cv::videostab::NullWobbleSuppressor* instance) {
		delete instance;
	}
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:98
	void cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(cv::videostab::NullWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result, Result_void* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_OnePassStabilizer_delete(cv::videostab::OnePassStabilizer* instance) {
		delete instance;
	}
	// OnePassStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:146
	void cv_videostab_OnePassStabilizer_OnePassStabilizer(Result<cv::videostab::OnePassStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::OnePassStabilizer* ret = new cv::videostab::OnePassStabilizer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::OnePassStabilizer*>))
	}
	
	// setMotionFilter(Ptr<cv::videostab::MotionFilterBase>) /usr/include/opencv2/videostab/stabilizer.hpp:148
	void cv_videostab_OnePassStabilizer_setMotionFilter_Ptr_MotionFilterBase_(cv::videostab::OnePassStabilizer* instance, cv::Ptr<cv::videostab::MotionFilterBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotionFilter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionFilter() /usr/include/opencv2/videostab/stabilizer.hpp:149
	void cv_videostab_OnePassStabilizer_motionFilter_const(const cv::videostab::OnePassStabilizer* instance, Result<cv::Ptr<cv::videostab::MotionFilterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::MotionFilterBase> ret = instance->motionFilter();
			Ok(new cv::Ptr<cv::videostab::MotionFilterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::MotionFilterBase>*>))
	}
	
	// reset() /usr/include/opencv2/videostab/stabilizer.hpp:151
	void cv_videostab_OnePassStabilizer_reset(cv::videostab::OnePassStabilizer* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/stabilizer.hpp:152
	void cv_videostab_OnePassStabilizer_nextFrame(cv::videostab::OnePassStabilizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimator(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		return dynamic_cast<cv::videostab::SparsePyrLkOptFlowEstimator*>(instance);
	}
	
	void cv_PyrLkOptFlowEstimatorBase_delete(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
		delete instance;
	}
	// PyrLkOptFlowEstimatorBase() /usr/include/opencv2/videostab/optical_flow.hpp:82
	void cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase(Result<cv::videostab::PyrLkOptFlowEstimatorBase*>* ocvrs_return) {
		try {
			cv::videostab::PyrLkOptFlowEstimatorBase* ret = new cv::videostab::PyrLkOptFlowEstimatorBase();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::PyrLkOptFlowEstimatorBase*>))
	}
	
	// setWinSize(cv::Size) /usr/include/opencv2/videostab/optical_flow.hpp:84
	void cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(cv::videostab::PyrLkOptFlowEstimatorBase* instance, cv::Size* val, Result_void* ocvrs_return) {
		try {
			instance->setWinSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// winSize() /usr/include/opencv2/videostab/optical_flow.hpp:85
	void cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->winSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setMaxLevel(int) /usr/include/opencv2/videostab/optical_flow.hpp:87
	void cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(cv::videostab::PyrLkOptFlowEstimatorBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// maxLevel() /usr/include/opencv2/videostab/optical_flow.hpp:88
	void cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// size /usr/include/opencv2/videostab/motion_core.hpp:75
	int cv_videostab_RansacParams_getPropSize_const(const cv::videostab::RansacParams* instance) {
			int ret = instance->size;
			return ret;
	}
	
	// size /usr/include/opencv2/videostab/motion_core.hpp:75
	void cv_videostab_RansacParams_setPropSize_int(cv::videostab::RansacParams* instance, int val) {
			instance->size = val;
	}
	
	// thresh /usr/include/opencv2/videostab/motion_core.hpp:76
	float cv_videostab_RansacParams_getPropThresh_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->thresh;
			return ret;
	}
	
	// thresh /usr/include/opencv2/videostab/motion_core.hpp:76
	void cv_videostab_RansacParams_setPropThresh_float(cv::videostab::RansacParams* instance, float val) {
			instance->thresh = val;
	}
	
	// eps /usr/include/opencv2/videostab/motion_core.hpp:77
	float cv_videostab_RansacParams_getPropEps_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->eps;
			return ret;
	}
	
	// eps /usr/include/opencv2/videostab/motion_core.hpp:77
	void cv_videostab_RansacParams_setPropEps_float(cv::videostab::RansacParams* instance, float val) {
			instance->eps = val;
	}
	
	// prob /usr/include/opencv2/videostab/motion_core.hpp:78
	float cv_videostab_RansacParams_getPropProb_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->prob;
			return ret;
	}
	
	// prob /usr/include/opencv2/videostab/motion_core.hpp:78
	void cv_videostab_RansacParams_setPropProb_float(cv::videostab::RansacParams* instance, float val) {
			instance->prob = val;
	}
	
	void cv_RansacParams_delete(cv::videostab::RansacParams* instance) {
		delete instance;
	}
	// RansacParams() /usr/include/opencv2/videostab/motion_core.hpp:80
	void cv_videostab_RansacParams_RansacParams(Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::RansacParams*>))
	}
	
	// RansacParams(int, float, float, float) /usr/include/opencv2/videostab/motion_core.hpp:87
	void cv_videostab_RansacParams_RansacParams_int_float_float_float(int size, float thresh, float eps, float prob, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams(size, thresh, eps, prob);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::RansacParams*>))
	}
	
	// niters() /usr/include/opencv2/videostab/motion_core.hpp:92
	void cv_videostab_RansacParams_niters_const(const cv::videostab::RansacParams* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->niters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// default2dMotion(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_core.hpp:102
	void cv_videostab_RansacParams_default2dMotion_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = cv::videostab::RansacParams::default2dMotion(model);
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::RansacParams*>))
	}
	
	cv::videostab::PyrLkOptFlowEstimatorBase* cv_SparsePyrLkOptFlowEstimator_to_PyrLkOptFlowEstimatorBase(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
		return dynamic_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance);
	}
	
	void cv_SparsePyrLkOptFlowEstimator_delete(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
		delete instance;
	}
	// run(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:100
	void cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::videostab::SparsePyrLkOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors, Result_void* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLog(Ptr<cv::videostab::ILog>) /usr/include/opencv2/videostab/stabilizer.hpp:71
	void cv_videostab_StabilizerBase_setLog_Ptr_ILog_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ILog>* ilog, Result_void* ocvrs_return) {
		try {
			instance->setLog(*ilog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// log() /usr/include/opencv2/videostab/stabilizer.hpp:72
	void cv_videostab_StabilizerBase_log_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::ILog>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ILog> ret = instance->log();
			Ok(new cv::Ptr<cv::videostab::ILog>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::ILog>*>))
	}
	
	// setRadius(int) /usr/include/opencv2/videostab/stabilizer.hpp:74
	void cv_videostab_StabilizerBase_setRadius_int(cv::videostab::StabilizerBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// radius() /usr/include/opencv2/videostab/stabilizer.hpp:75
	void cv_videostab_StabilizerBase_radius_const(const cv::videostab::StabilizerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFrameSource(Ptr<cv::videostab::IFrameSource>) /usr/include/opencv2/videostab/stabilizer.hpp:77
	void cv_videostab_StabilizerBase_setFrameSource_Ptr_IFrameSource_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::IFrameSource>* val, Result_void* ocvrs_return) {
		try {
			instance->setFrameSource(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// frameSource() /usr/include/opencv2/videostab/stabilizer.hpp:78
	void cv_videostab_StabilizerBase_frameSource_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::IFrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IFrameSource> ret = instance->frameSource();
			Ok(new cv::Ptr<cv::videostab::IFrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::IFrameSource>*>))
	}
	
	// setMaskSource(const Ptr<cv::videostab::IFrameSource> &) /usr/include/opencv2/videostab/stabilizer.hpp:80
	void cv_videostab_StabilizerBase_setMaskSource_const_Ptr_IFrameSource_R(cv::videostab::StabilizerBase* instance, const cv::Ptr<cv::videostab::IFrameSource>* val, Result_void* ocvrs_return) {
		try {
			instance->setMaskSource(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// maskSource() /usr/include/opencv2/videostab/stabilizer.hpp:81
	void cv_videostab_StabilizerBase_maskSource_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::IFrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IFrameSource> ret = instance->maskSource();
			Ok(new cv::Ptr<cv::videostab::IFrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::IFrameSource>*>))
	}
	
	// setMotionEstimator(Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/stabilizer.hpp:83
	void cv_videostab_StabilizerBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotionEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionEstimator() /usr/include/opencv2/videostab/stabilizer.hpp:84
	void cv_videostab_StabilizerBase_motionEstimator_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>))
	}
	
	// setDeblurer(Ptr<cv::videostab::DeblurerBase>) /usr/include/opencv2/videostab/stabilizer.hpp:86
	void cv_videostab_StabilizerBase_setDeblurer_Ptr_DeblurerBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::DeblurerBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setDeblurer(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// deblurrer() /usr/include/opencv2/videostab/stabilizer.hpp:87
	void cv_videostab_StabilizerBase_deblurrer_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::DeblurerBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::DeblurerBase> ret = instance->deblurrer();
			Ok(new cv::Ptr<cv::videostab::DeblurerBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::DeblurerBase>*>))
	}
	
	// setTrimRatio(float) /usr/include/opencv2/videostab/stabilizer.hpp:89
	void cv_videostab_StabilizerBase_setTrimRatio_float(cv::videostab::StabilizerBase* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// trimRatio() /usr/include/opencv2/videostab/stabilizer.hpp:90
	void cv_videostab_StabilizerBase_trimRatio_const(const cv::videostab::StabilizerBase* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->trimRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setCorrectionForInclusion(bool) /usr/include/opencv2/videostab/stabilizer.hpp:92
	void cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(cv::videostab::StabilizerBase* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setCorrectionForInclusion(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// doCorrectionForInclusion() /usr/include/opencv2/videostab/stabilizer.hpp:93
	void cv_videostab_StabilizerBase_doCorrectionForInclusion_const(const cv::videostab::StabilizerBase* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->doCorrectionForInclusion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setBorderMode(int) /usr/include/opencv2/videostab/stabilizer.hpp:95
	void cv_videostab_StabilizerBase_setBorderMode_int(cv::videostab::StabilizerBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setBorderMode(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// borderMode() /usr/include/opencv2/videostab/stabilizer.hpp:96
	void cv_videostab_StabilizerBase_borderMode_const(const cv::videostab::StabilizerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->borderMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setInpainter(Ptr<cv::videostab::InpainterBase>) /usr/include/opencv2/videostab/stabilizer.hpp:98
	void cv_videostab_StabilizerBase_setInpainter_Ptr_InpainterBase_(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::InpainterBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setInpainter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// inpainter() /usr/include/opencv2/videostab/stabilizer.hpp:99
	void cv_videostab_StabilizerBase_inpainter_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::InpainterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::InpainterBase> ret = instance->inpainter();
			Ok(new cv::Ptr<cv::videostab::InpainterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::InpainterBase>*>))
	}
	
	void cv_ToFileMotionWriter_delete(cv::videostab::ToFileMotionWriter* instance) {
		delete instance;
	}
	// ToFileMotionWriter(const cv::String &, Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/global_motion.hpp:212
	void cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_Ptr_ImageMotionEstimatorBase_(const char* path, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* estimator, Result<cv::videostab::ToFileMotionWriter*>* ocvrs_return) {
		try {
			cv::videostab::ToFileMotionWriter* ret = new cv::videostab::ToFileMotionWriter(std::string(path), *estimator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::ToFileMotionWriter*>))
	}
	
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:214
	void cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(cv::videostab::ToFileMotionWriter* instance, cv::videostab::MotionModel val, Result_void* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:215
	void cv_videostab_ToFileMotionWriter_motionModel_const(const cv::videostab::ToFileMotionWriter* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::MotionModel>))
	}
	
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:217
	void cv_videostab_ToFileMotionWriter_setFrameMask_const__InputArrayR(cv::videostab::ToFileMotionWriter* instance, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->setFrameMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:219
	void cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(cv::videostab::ToFileMotionWriter* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_TranslationBasedLocalOutlierRejector_delete(cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
		delete instance;
	}
	// TranslationBasedLocalOutlierRejector() /usr/include/opencv2/videostab/outlier_rejection.hpp:77
	void cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector(Result<cv::videostab::TranslationBasedLocalOutlierRejector*>* ocvrs_return) {
		try {
			cv::videostab::TranslationBasedLocalOutlierRejector* ret = new cv::videostab::TranslationBasedLocalOutlierRejector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::TranslationBasedLocalOutlierRejector*>))
	}
	
	// setCellSize(cv::Size) /usr/include/opencv2/videostab/outlier_rejection.hpp:79
	void cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::Size* val, Result_void* ocvrs_return) {
		try {
			instance->setCellSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cellSize() /usr/include/opencv2/videostab/outlier_rejection.hpp:80
	void cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->cellSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setRansacParams(cv::videostab::RansacParams) /usr/include/opencv2/videostab/outlier_rejection.hpp:82
	void cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::videostab::RansacParams* val, Result_void* ocvrs_return) {
		try {
			instance->setRansacParams(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ransacParams() /usr/include/opencv2/videostab/outlier_rejection.hpp:83
	void cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::RansacParams*>))
	}
	
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:85
	void cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TwoPassStabilizer_delete(cv::videostab::TwoPassStabilizer* instance) {
		delete instance;
	}
	// TwoPassStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:166
	void cv_videostab_TwoPassStabilizer_TwoPassStabilizer(Result<cv::videostab::TwoPassStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::TwoPassStabilizer* ret = new cv::videostab::TwoPassStabilizer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::TwoPassStabilizer*>))
	}
	
	// setMotionStabilizer(Ptr<cv::videostab::IMotionStabilizer>) /usr/include/opencv2/videostab/stabilizer.hpp:168
	void cv_videostab_TwoPassStabilizer_setMotionStabilizer_Ptr_IMotionStabilizer_(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotionStabilizer(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:169
	void cv_videostab_TwoPassStabilizer_motionStabilizer_const(const cv::videostab::TwoPassStabilizer* instance, Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IMotionStabilizer> ret = instance->motionStabilizer();
			Ok(new cv::Ptr<cv::videostab::IMotionStabilizer>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>))
	}
	
	// setWobbleSuppressor(Ptr<cv::videostab::WobbleSuppressorBase>) /usr/include/opencv2/videostab/stabilizer.hpp:171
	void cv_videostab_TwoPassStabilizer_setWobbleSuppressor_Ptr_WobbleSuppressorBase_(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::WobbleSuppressorBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setWobbleSuppressor(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// wobbleSuppressor() /usr/include/opencv2/videostab/stabilizer.hpp:172
	void cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(const cv::videostab::TwoPassStabilizer* instance, Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::WobbleSuppressorBase> ret = instance->wobbleSuppressor();
			Ok(new cv::Ptr<cv::videostab::WobbleSuppressorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>))
	}
	
	// setEstimateTrimRatio(bool) /usr/include/opencv2/videostab/stabilizer.hpp:174
	void cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(cv::videostab::TwoPassStabilizer* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setEstimateTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mustEstimateTrimaRatio() /usr/include/opencv2/videostab/stabilizer.hpp:175
	void cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(const cv::videostab::TwoPassStabilizer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->mustEstimateTrimaRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// reset() /usr/include/opencv2/videostab/stabilizer.hpp:177
	void cv_videostab_TwoPassStabilizer_reset(cv::videostab::TwoPassStabilizer* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/stabilizer.hpp:178
	void cv_videostab_TwoPassStabilizer_nextFrame(cv::videostab::TwoPassStabilizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_VideoFileSource_delete(cv::videostab::VideoFileSource* instance) {
		delete instance;
	}
	// VideoFileSource(const cv::String &, bool) /usr/include/opencv2/videostab/frame_source.hpp:75
	void cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(const char* path, bool volatileFrame, Result<cv::videostab::VideoFileSource*>* ocvrs_return) {
		try {
			cv::videostab::VideoFileSource* ret = new cv::videostab::VideoFileSource(std::string(path), volatileFrame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::VideoFileSource*>))
	}
	
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:77
	void cv_videostab_VideoFileSource_reset(cv::videostab::VideoFileSource* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:78
	void cv_videostab_VideoFileSource_nextFrame(cv::videostab::VideoFileSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// width() /usr/include/opencv2/videostab/frame_source.hpp:80
	void cv_videostab_VideoFileSource_width(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->width();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// height() /usr/include/opencv2/videostab/frame_source.hpp:81
	void cv_videostab_VideoFileSource_height(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->height();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// count() /usr/include/opencv2/videostab/frame_source.hpp:82
	void cv_videostab_VideoFileSource_count(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->count();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// fps() /usr/include/opencv2/videostab/frame_source.hpp:83
	void cv_videostab_VideoFileSource_fps(cv::videostab::VideoFileSource* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->fps();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_WeightingDeblurer_delete(cv::videostab::WeightingDeblurer* instance) {
		delete instance;
	}
	// WeightingDeblurer() /usr/include/opencv2/videostab/deblurring.hpp:99
	void cv_videostab_WeightingDeblurer_WeightingDeblurer(Result<cv::videostab::WeightingDeblurer*>* ocvrs_return) {
		try {
			cv::videostab::WeightingDeblurer* ret = new cv::videostab::WeightingDeblurer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::videostab::WeightingDeblurer*>))
	}
	
	// setSensitivity(float) /usr/include/opencv2/videostab/deblurring.hpp:101
	void cv_videostab_WeightingDeblurer_setSensitivity_float(cv::videostab::WeightingDeblurer* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setSensitivity(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// sensitivity() /usr/include/opencv2/videostab/deblurring.hpp:102
	void cv_videostab_WeightingDeblurer_sensitivity_const(const cv::videostab::WeightingDeblurer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->sensitivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:104
	void cv_videostab_WeightingDeblurer_deblur_int_MatR_const_RangeR(cv::videostab::WeightingDeblurer* instance, int idx, cv::Mat* frame, const cv::Range* range, Result_void* ocvrs_return) {
		try {
			instance->deblur(idx, *frame, *range);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMotionEstimator(Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/wobble_suppression.hpp:67
	void cv_videostab_WobbleSuppressorBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(cv::videostab::WobbleSuppressorBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotionEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motionEstimator() /usr/include/opencv2/videostab/wobble_suppression.hpp:68
	void cv_videostab_WobbleSuppressorBase_motionEstimator_const(const cv::videostab::WobbleSuppressorBase* instance, Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>))
	}
	
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:70
	void cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(cv::videostab::WobbleSuppressorBase* instance, int idx, const cv::Mat* frame, cv::Mat* result, Result_void* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFrameCount(int) /usr/include/opencv2/videostab/wobble_suppression.hpp:75
	void cv_videostab_WobbleSuppressorBase_setFrameCount_int(cv::videostab::WobbleSuppressorBase* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setFrameCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// frameCount() /usr/include/opencv2/videostab/wobble_suppression.hpp:76
	void cv_videostab_WobbleSuppressorBase_frameCount_const(const cv::videostab::WobbleSuppressorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->frameCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:78
	void cv_videostab_WobbleSuppressorBase_setMotions_const_vector_Mat_R(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motions() /usr/include/opencv2/videostab/wobble_suppression.hpp:79
	void cv_videostab_WobbleSuppressorBase_motions_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setMotions2(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:81
	void cv_videostab_WobbleSuppressorBase_setMotions2_const_vector_Mat_R(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setMotions2(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// motions2() /usr/include/opencv2/videostab/wobble_suppression.hpp:82
	void cv_videostab_WobbleSuppressorBase_motions2_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions2();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:84
	void cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vector_Mat_R(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, Result_void* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stabilizationMotions() /usr/include/opencv2/videostab/wobble_suppression.hpp:85
	void cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizationMotions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
}
