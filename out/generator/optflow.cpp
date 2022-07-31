#include "ocvrs_common.hpp"
#include <opencv2/optflow.hpp>
#include "optflow_types.hpp"

extern "C" {
	// calcGlobalOrientation(cv::InputArray, cv::InputArray, cv::InputArray, double, double) /usr/include/opencv2/optflow/motempl.hpp:119
	void cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(const cv::_InputArray* orientation, const cv::_InputArray* mask, const cv::_InputArray* mhi, double timestamp, double duration, Result<double>* ocvrs_return) {
		try {
			double ret = cv::motempl::calcGlobalOrientation(*orientation, *mask, *mhi, timestamp, duration);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// calcMotionGradient(cv::InputArray, cv::OutputArray, cv::OutputArray, double, double, int) /usr/include/opencv2/optflow/motempl.hpp:102
	void cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* mhi, const cv::_OutputArray* mask, const cv::_OutputArray* orientation, double delta1, double delta2, int apertureSize, Result_void* ocvrs_return) {
		try {
			cv::motempl::calcMotionGradient(*mhi, *mask, *orientation, delta1, delta2, apertureSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// segmentMotion(cv::InputArray, cv::OutputArray, std::vector<Rect> &, double, double) /usr/include/opencv2/optflow/motempl.hpp:137
	void cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vector_Rect_R_double_double(const cv::_InputArray* mhi, const cv::_OutputArray* segmask, std::vector<cv::Rect>* boundingRects, double timestamp, double segThresh, Result_void* ocvrs_return) {
		try {
			cv::motempl::segmentMotion(*mhi, *segmask, *boundingRects, timestamp, segThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updateMotionHistory(cv::InputArray, cv::InputOutputArray, double, double) /usr/include/opencv2/optflow/motempl.hpp:71
	void cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(const cv::_InputArray* silhouette, const cv::_InputOutputArray* mhi, double timestamp, double duration, Result_void* ocvrs_return) {
		try {
			cv::motempl::updateMotionHistory(*silhouette, *mhi, timestamp, duration);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowDenseRLOF(cv::InputArray, cv::InputArray, cv::InputOutputArray, Ptr<cv::optflow::RLOFOpticalFlowParameter>, float, cv::Size, cv::optflow::InterpolationType, int, float, float, int, int, bool, float, float, bool) /usr/include/opencv2/optflow/rlofflow.hpp:501
	void cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement, Result_void* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowDenseRLOF(*I0, *I1, *flow, *rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowSF(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/optflow.hpp:81
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, Result_void* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowSF(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double) /usr/include/opencv2/optflow.hpp:110
	void cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int layers, int averaging_block_size, int max_flow, double sigma_dist, double sigma_color, int postprocess_window, double sigma_dist_fix, double sigma_color_fix, double occ_thr, int upscale_averaging_radius, double upscale_sigma_dist, double upscale_sigma_color, double speed_up_thr, Result_void* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSF(*from, *to, *flow, layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowSparseRLOF(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, Ptr<cv::optflow::RLOFOpticalFlowParameter>, float) /usr/include/opencv2/optflow/rlofflow.hpp:538
	void cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_RLOFOpticalFlowParameter__float(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, Result_void* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseRLOF(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *rlofParam, forwardBackwardThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calcOpticalFlowSparseToDense(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool, float, float) /usr/include/opencv2/optflow.hpp:135
	void cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(const cv::_InputArray* from, const cv::_InputArray* to, const cv::_OutputArray* flow, int grid_step, int k, float sigma, bool use_post_proc, float fgs_lambda, float fgs_sigma, Result_void* ocvrs_return) {
		try {
			cv::optflow::calcOpticalFlowSparseToDense(*from, *to, *flow, grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createOptFlow_DeepFlow() /usr/include/opencv2/optflow.hpp:165
	void cv_optflow_createOptFlow_DeepFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DeepFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// createOptFlow_DenseRLOF() /usr/include/opencv2/optflow/rlofflow.hpp:545
	void cv_optflow_createOptFlow_DenseRLOF(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_DenseRLOF();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// createOptFlow_DualTVL1() /usr/include/opencv2/optflow.hpp:300
	void cv_optflow_createOptFlow_DualTVL1(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::createOptFlow_DualTVL1();
			Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>))
	}
	
	// createOptFlow_Farneback() /usr/include/opencv2/optflow.hpp:171
	void cv_optflow_createOptFlow_Farneback(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_Farneback();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// createOptFlow_PCAFlow() /usr/include/opencv2/optflow/pcaflow.hpp:142
	void cv_optflow_createOptFlow_PCAFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_PCAFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// createOptFlow_SimpleFlow() /usr/include/opencv2/optflow.hpp:168
	void cv_optflow_createOptFlow_SimpleFlow(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SimpleFlow();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// createOptFlow_SparseRLOF() /usr/include/opencv2/optflow/rlofflow.hpp:548
	void cv_optflow_createOptFlow_SparseRLOF(Result<cv::Ptr<cv::SparseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparseOpticalFlow> ret = cv::optflow::createOptFlow_SparseRLOF();
			Ok(new cv::Ptr<cv::SparseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::SparseOpticalFlow>*>))
	}
	
	// createOptFlow_SparseToDense() /usr/include/opencv2/optflow.hpp:174
	void cv_optflow_createOptFlow_SparseToDense(Result<cv::Ptr<cv::DenseOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DenseOpticalFlow> ret = cv::optflow::createOptFlow_SparseToDense();
			Ok(new cv::Ptr<cv::DenseOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DenseOpticalFlow>*>))
	}
	
	// read(const cv::FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:369
	void cv_read_const_FileNodeR_NodeR_Node(const cv::FileNode* fn, cv::optflow::GPCTree::Node* node, cv::optflow::GPCTree::Node* unnamed, Result_void* ocvrs_return) {
		try {
			cv::read(*fn, *node, *unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const optflow::GPCTree::Node &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:367
	void cv_write_FileStorageR_const_StringR_const_NodeR(cv::FileStorage* fs, const char* name, const cv::optflow::GPCTree::Node* node, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setRLOFOpticalFlowParameter(Ptr<cv::optflow::RLOFOpticalFlowParameter>) /usr/include/opencv2/optflow/rlofflow.hpp:240
	void cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(cv::optflow::DenseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val, Result_void* ocvrs_return) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:244
	void cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	// setForwardBackward(float) /usr/include/opencv2/optflow/rlofflow.hpp:253
	void cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setForwardBackward(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getForwardBackward() /usr/include/opencv2/optflow/rlofflow.hpp:257
	void cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getForwardBackward();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getGridStep() /usr/include/opencv2/optflow/rlofflow.hpp:263
	void cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getGridStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setGridStep(cv::Size) /usr/include/opencv2/optflow/rlofflow.hpp:267
	void cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(cv::optflow::DenseRLOFOpticalFlow* instance, cv::Size* val, Result_void* ocvrs_return) {
		try {
			instance->setGridStep(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInterpolation(cv::optflow::InterpolationType) /usr/include/opencv2/optflow/rlofflow.hpp:275
	void cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(cv::optflow::DenseRLOFOpticalFlow* instance, cv::optflow::InterpolationType val, Result_void* ocvrs_return) {
		try {
			instance->setInterpolation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInterpolation() /usr/include/opencv2/optflow/rlofflow.hpp:279
	void cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<cv::optflow::InterpolationType>* ocvrs_return) {
		try {
			cv::optflow::InterpolationType ret = instance->getInterpolation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::InterpolationType>))
	}
	
	// getEPICK() /usr/include/opencv2/optflow/rlofflow.hpp:285
	void cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEPICK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setEPICK(int) /usr/include/opencv2/optflow/rlofflow.hpp:289
	void cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setEPICK(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEPICSigma() /usr/include/opencv2/optflow/rlofflow.hpp:296
	void cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEPICSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setEPICSigma(float) /usr/include/opencv2/optflow/rlofflow.hpp:300
	void cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setEPICSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEPICLambda() /usr/include/opencv2/optflow/rlofflow.hpp:306
	void cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEPICLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setEPICLambda(float) /usr/include/opencv2/optflow/rlofflow.hpp:310
	void cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setEPICLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFgsLambda() /usr/include/opencv2/optflow/rlofflow.hpp:315
	void cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFgsLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setFgsLambda(float) /usr/include/opencv2/optflow/rlofflow.hpp:319
	void cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setFgsLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFgsSigma() /usr/include/opencv2/optflow/rlofflow.hpp:324
	void cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFgsSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setFgsSigma(float) /usr/include/opencv2/optflow/rlofflow.hpp:328
	void cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(cv::optflow::DenseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setFgsSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUsePostProc(bool) /usr/include/opencv2/optflow/rlofflow.hpp:333
	void cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUsePostProc(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUsePostProc() /usr/include/opencv2/optflow/rlofflow.hpp:337
	void cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUsePostProc();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseVariationalRefinement(bool) /usr/include/opencv2/optflow/rlofflow.hpp:342
	void cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(cv::optflow::DenseRLOFOpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseVariationalRefinement() /usr/include/opencv2/optflow/rlofflow.hpp:346
	void cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setRICSPSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:351
	void cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRICSPSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRICSPSize() /usr/include/opencv2/optflow/rlofflow.hpp:355
	void cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRICSPSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRICSLICType(int) /usr/include/opencv2/optflow/rlofflow.hpp:362
	void cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(cv::optflow::DenseRLOFOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRICSLICType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRICSLICType() /usr/include/opencv2/optflow/rlofflow.hpp:366
	void cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(const cv::optflow::DenseRLOFOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRICSLICType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// create(Ptr<cv::optflow::RLOFOpticalFlowParameter>, float, cv::Size, cv::optflow::InterpolationType, int, float, float, int, int, bool, float, float, bool) /usr/include/opencv2/optflow/rlofflow.hpp:383
	void cv_optflow_DenseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, cv::Size* gridStep, cv::optflow::InterpolationType interp_type, int epicK, float epicSigma, float epicLambda, int ricSPSize, int ricSLICType, bool use_post_proc, float fgsLambda, float fgsSigma, bool use_variational_refinement, Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DenseRLOFOpticalFlow> ret = cv::optflow::DenseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold, *gridStep, interp_type, epicK, epicSigma, epicLambda, ricSPSize, ricSLICType, use_post_proc, fgsLambda, fgsSigma, use_variational_refinement);
			Ok(new cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>))
	}
	
	// getTau() /usr/include/opencv2/optflow.hpp:223
	void cv_optflow_DualTVL1OpticalFlow_getTau_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setTau(double) /usr/include/opencv2/optflow.hpp:225
	void cv_optflow_DualTVL1OpticalFlow_setTau_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLambda() /usr/include/opencv2/optflow.hpp:228
	void cv_optflow_DualTVL1OpticalFlow_getLambda_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setLambda(double) /usr/include/opencv2/optflow.hpp:230
	void cv_optflow_DualTVL1OpticalFlow_setLambda_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTheta() /usr/include/opencv2/optflow.hpp:233
	void cv_optflow_DualTVL1OpticalFlow_getTheta_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setTheta(double) /usr/include/opencv2/optflow.hpp:235
	void cv_optflow_DualTVL1OpticalFlow_setTheta_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setTheta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGamma() /usr/include/opencv2/optflow.hpp:238
	void cv_optflow_DualTVL1OpticalFlow_getGamma_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setGamma(double) /usr/include/opencv2/optflow.hpp:240
	void cv_optflow_DualTVL1OpticalFlow_setGamma_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScalesNumber() /usr/include/opencv2/optflow.hpp:243
	void cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScalesNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setScalesNumber(int) /usr/include/opencv2/optflow.hpp:245
	void cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setScalesNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWarpingsNumber() /usr/include/opencv2/optflow.hpp:248
	void cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWarpingsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWarpingsNumber(int) /usr/include/opencv2/optflow.hpp:250
	void cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWarpingsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEpsilon() /usr/include/opencv2/optflow.hpp:253
	void cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setEpsilon(double) /usr/include/opencv2/optflow.hpp:255
	void cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInnerIterations() /usr/include/opencv2/optflow.hpp:258
	void cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setInnerIterations(int) /usr/include/opencv2/optflow.hpp:260
	void cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setInnerIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOuterIterations() /usr/include/opencv2/optflow.hpp:263
	void cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setOuterIterations(int) /usr/include/opencv2/optflow.hpp:265
	void cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setOuterIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseInitialFlow() /usr/include/opencv2/optflow.hpp:268
	void cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseInitialFlow(bool) /usr/include/opencv2/optflow.hpp:270
	void cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::optflow::DualTVL1OpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScaleStep() /usr/include/opencv2/optflow.hpp:273
	void cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setScaleStep(double) /usr/include/opencv2/optflow.hpp:275
	void cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(cv::optflow::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setScaleStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMedianFiltering() /usr/include/opencv2/optflow.hpp:278
	void cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(const cv::optflow::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMedianFiltering();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMedianFiltering(int) /usr/include/opencv2/optflow.hpp:280
	void cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(cv::optflow::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMedianFiltering(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(double, double, double, int, int, double, int, int, double, double, int, bool) /usr/include/opencv2/optflow.hpp:283
	void cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow, Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::DualTVL1OpticalFlow> ret = cv::optflow::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			Ok(new cv::Ptr<cv::optflow::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>))
	}
	
	void cv_GPCDetails_delete(cv::optflow::GPCDetails* instance) {
		delete instance;
	}
	// getAllDescriptorsForImage(const cv::Mat *, std::vector<GPCPatchDescriptor> &, const cv::optflow::GPCMatchingParams &, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:306
	void cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_R_const_GPCMatchingParamsR_int(const cv::Mat* imgCh, std::vector<cv::optflow::GPCPatchDescriptor>* descr, const cv::optflow::GPCMatchingParams* mp, int type, Result_void* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::getAllDescriptorsForImage(imgCh, *descr, *mp, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCoordinatesFromIndex(size_t, cv::Size, int &, int &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:309
	void cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(size_t index, cv::Size* sz, int* x, int* y, Result_void* ocvrs_return) {
		try {
			cv::optflow::GPCDetails::getCoordinatesFromIndex(index, *sz, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GPCMatchingParams(bool) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:147
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(bool _useOpenCL, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(_useOpenCL);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	// GPCMatchingParams(const cv::optflow::GPCMatchingParams &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:149
	void cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(const cv::optflow::GPCMatchingParams* params, Result<cv::optflow::GPCMatchingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCMatchingParams ret(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCMatchingParams>))
	}
	
	// feature /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	void cv_optflow_GPCPatchDescriptor_getPropFeature_const(const cv::optflow::GPCPatchDescriptor* instance, cv::Vec<double, 18>* ocvrs_return) {
			cv::Vec<double, 18> ret = instance->feature;
			*ocvrs_return = ret;
	}
	
	// feature /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	void cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(cv::optflow::GPCPatchDescriptor* instance, cv::Vec<double, 18>* val) {
			instance->feature = *val;
	}
	
	void cv_GPCPatchDescriptor_delete(cv::optflow::GPCPatchDescriptor* instance) {
		delete instance;
	}
	// dot(const Vec<double, nFeatures> &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:70
	void cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_R(const cv::optflow::GPCPatchDescriptor* instance, const cv::Vec<double, 18>* coef, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*coef);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// markAsSeparated() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:72
	void cv_optflow_GPCPatchDescriptor_markAsSeparated(cv::optflow::GPCPatchDescriptor* instance, Result_void* ocvrs_return) {
		try {
			instance->markAsSeparated();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isSeparated() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:74
	void cv_optflow_GPCPatchDescriptor_isSeparated_const(const cv::optflow::GPCPatchDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSeparated();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ref /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_getPropRef_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->ref;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}
	
	// ref /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	void cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
			instance->ref = *val;
	}
	
	// pos /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_getPropPos_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->pos;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}
	
	// pos /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	void cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
			instance->pos = *val;
	}
	
	// neg /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	cv::optflow::GPCPatchDescriptor* cv_optflow_GPCPatchSample_getPropNeg_const(const cv::optflow::GPCPatchSample* instance) {
			cv::optflow::GPCPatchDescriptor ret = instance->neg;
			return new cv::optflow::GPCPatchDescriptor(ret);
	}
	
	// neg /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	void cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(cv::optflow::GPCPatchSample* instance, cv::optflow::GPCPatchDescriptor* val) {
			instance->neg = *val;
	}
	
	void cv_GPCPatchSample_delete(cv::optflow::GPCPatchSample* instance) {
		delete instance;
	}
	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:83
	void cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_R_double(const cv::optflow::GPCPatchSample* instance, bool* refdir, bool* posdir, bool* negdir, const cv::Vec<double, 18>* coef, double rhs, Result_void* ocvrs_return) {
		try {
			instance->getDirections(*refdir, *posdir, *negdir, *coef, rhs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GPCTrainingParams(unsigned int, int, cv::optflow::GPCDescType, bool) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:130
	void cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(unsigned int _maxTreeDepth, int _minNumberOfSamples, cv::optflow::GPCDescType _descriptorType, bool _printProgress, Result<cv::optflow::GPCTrainingParams>* ocvrs_return) {
		try {
			cv::optflow::GPCTrainingParams ret(_maxTreeDepth, _minNumberOfSamples, _descriptorType, _printProgress);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::GPCTrainingParams>))
	}
	
	// check() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:138
	void cv_optflow_GPCTrainingParams_check_const(const cv::optflow::GPCTrainingParams instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance.check();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_GPCTrainingSamples_delete(cv::optflow::GPCTrainingSamples* instance) {
		delete instance;
	}
	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:108
	void cv_optflow_GPCTrainingSamples_create_const_vector_String_R_const_vector_String_R_const_vector_String_R_int(const std::vector<cv::String>* imagesFrom, const std::vector<cv::String>* imagesTo, const std::vector<cv::String>* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	// create(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::InputArrayOfArrays, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:111
	void cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* imagesFrom, const cv::_InputArray* imagesTo, const cv::_InputArray* gt, int descriptorType, Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTrainingSamples> ret = cv::optflow::GPCTrainingSamples::create(*imagesFrom, *imagesTo, *gt, descriptorType);
			Ok(new cv::Ptr<cv::optflow::GPCTrainingSamples>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>))
	}
	
	// size() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:114
	void cv_optflow_GPCTrainingSamples_size_const(const cv::optflow::GPCTrainingSamples* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// type() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:116
	void cv_optflow_GPCTrainingSamples_type_const(const cv::optflow::GPCTrainingSamples* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	cv::Algorithm* cv_GPCTree_to_Algorithm(cv::optflow::GPCTree* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_GPCTree_delete(cv::optflow::GPCTree* instance) {
		delete instance;
	}
	// train(cv::optflow::GPCTrainingSamples &, const cv::optflow::GPCTrainingParams) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:176
	void cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(cv::optflow::GPCTree* instance, cv::optflow::GPCTrainingSamples* samples, const cv::optflow::GPCTrainingParams* params, Result_void* ocvrs_return) {
		try {
			instance->train(*samples, *params);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:178
	void cv_optflow_GPCTree_write_const_FileStorageR(const cv::optflow::GPCTree* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:180
	void cv_optflow_GPCTree_read_const_FileNodeR(cv::optflow::GPCTree* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findLeafForPatch(const cv::optflow::GPCPatchDescriptor &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:182
	void cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCPatchDescriptor* descr, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->findLeafForPatch(*descr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// create() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:184
	void cv_optflow_GPCTree_create(Result<cv::Ptr<cv::optflow::GPCTree>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::GPCTree> ret = cv::optflow::GPCTree::create();
			Ok(new cv::Ptr<cv::optflow::GPCTree>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::GPCTree>*>))
	}
	
	// operator==(const cv::optflow::GPCTree &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:186
	void cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(const cv::optflow::GPCTree* instance, const cv::optflow::GPCTree* t, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*t);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getDescriptorType() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:188
	void cv_optflow_GPCTree_getDescriptorType_const(const cv::optflow::GPCTree* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator==(const cv::optflow::GPCTree::Node &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:164
	void cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(const cv::optflow::GPCTree::Node instance, const cv::optflow::GPCTree::Node* n, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance.operator==(*n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	cv::Algorithm* cv_OpticalFlowPCAFlow_to_Algorithm(cv::optflow::OpticalFlowPCAFlow* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_OpticalFlowPCAFlow_delete(cv::optflow::OpticalFlowPCAFlow* instance) {
		delete instance;
	}
	// OpticalFlowPCAFlow(Ptr<const cv::optflow::PCAPrior>, const cv::Size, float, float, float, float, float) /usr/include/opencv2/optflow/pcaflow.hpp:116
	void cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_const_PCAPrior__const_Size_float_float_float_float_float(cv::Ptr<const cv::optflow::PCAPrior>* _prior, const cv::Size* _basisSize, float _sparseRate, float _retainedCornersFraction, float _occlusionsThreshold, float _dampingFactor, float _claheClip, Result<cv::optflow::OpticalFlowPCAFlow*>* ocvrs_return) {
		try {
			cv::optflow::OpticalFlowPCAFlow* ret = new cv::optflow::OpticalFlowPCAFlow(*_prior, *_basisSize, _sparseRate, _retainedCornersFraction, _occlusionsThreshold, _dampingFactor, _claheClip);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::OpticalFlowPCAFlow*>))
	}
	
	// calc(cv::InputArray, cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/optflow/pcaflow.hpp:120
	void cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::optflow::OpticalFlowPCAFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, Result_void* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// collectGarbage() /usr/include/opencv2/optflow/pcaflow.hpp:121
	void cv_optflow_OpticalFlowPCAFlow_collectGarbage(cv::optflow::OpticalFlowPCAFlow* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_PCAPrior_delete(cv::optflow::PCAPrior* instance) {
		delete instance;
	}
	// PCAPrior(const char *) /usr/include/opencv2/optflow/pcaflow.hpp:83
	void cv_optflow_PCAPrior_PCAPrior_const_charX(const char* pathToPrior, Result<cv::optflow::PCAPrior*>* ocvrs_return) {
		try {
			cv::optflow::PCAPrior* ret = new cv::optflow::PCAPrior(pathToPrior);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::PCAPrior*>))
	}
	
	// getPadding() /usr/include/opencv2/optflow/pcaflow.hpp:85
	void cv_optflow_PCAPrior_getPadding_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPadding();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getBasisSize() /usr/include/opencv2/optflow/pcaflow.hpp:87
	void cv_optflow_PCAPrior_getBasisSize_const(const cv::optflow::PCAPrior* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBasisSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// fillConstraints(float *, float *, float *, float *) /usr/include/opencv2/optflow/pcaflow.hpp:89
	void cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(const cv::optflow::PCAPrior* instance, float* A1, float* A2, float* b1, float* b2, Result_void* ocvrs_return) {
		try {
			instance->fillConstraints(A1, A2, b1, b2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// solverType /usr/include/opencv2/optflow/rlofflow.hpp:83
	void cv_optflow_RLOFOpticalFlowParameter_getPropSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType* ocvrs_return) {
			cv::optflow::SolverType ret = instance->solverType;
			*ocvrs_return = ret;
	}
	
	// solverType /usr/include/opencv2/optflow/rlofflow.hpp:83
	void cv_optflow_RLOFOpticalFlowParameter_setPropSolverType_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType val) {
			instance->solverType = val;
	}
	
	// supportRegionType /usr/include/opencv2/optflow/rlofflow.hpp:88
	void cv_optflow_RLOFOpticalFlowParameter_getPropSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType* ocvrs_return) {
			cv::optflow::SupportRegionType ret = instance->supportRegionType;
			*ocvrs_return = ret;
	}
	
	// supportRegionType /usr/include/opencv2/optflow/rlofflow.hpp:88
	void cv_optflow_RLOFOpticalFlowParameter_setPropSupportRegionType_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType val) {
			instance->supportRegionType = val;
	}
	
	// normSigma0 /usr/include/opencv2/optflow/rlofflow.hpp:92
	float cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->normSigma0;
			return ret;
	}
	
	// normSigma0 /usr/include/opencv2/optflow/rlofflow.hpp:92
	void cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma0_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
			instance->normSigma0 = val;
	}
	
	// normSigma1 /usr/include/opencv2/optflow/rlofflow.hpp:98
	float cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->normSigma1;
			return ret;
	}
	
	// normSigma1 /usr/include/opencv2/optflow/rlofflow.hpp:98
	void cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma1_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
			instance->normSigma1 = val;
	}
	
	// smallWinSize /usr/include/opencv2/optflow/rlofflow.hpp:104
	int cv_optflow_RLOFOpticalFlowParameter_getPropSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->smallWinSize;
			return ret;
	}
	
	// smallWinSize /usr/include/opencv2/optflow/rlofflow.hpp:104
	void cv_optflow_RLOFOpticalFlowParameter_setPropSmallWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
			instance->smallWinSize = val;
	}
	
	// largeWinSize /usr/include/opencv2/optflow/rlofflow.hpp:107
	int cv_optflow_RLOFOpticalFlowParameter_getPropLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->largeWinSize;
			return ret;
	}
	
	// largeWinSize /usr/include/opencv2/optflow/rlofflow.hpp:107
	void cv_optflow_RLOFOpticalFlowParameter_setPropLargeWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
			instance->largeWinSize = val;
	}
	
	// crossSegmentationThreshold /usr/include/opencv2/optflow/rlofflow.hpp:112
	int cv_optflow_RLOFOpticalFlowParameter_getPropCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->crossSegmentationThreshold;
			return ret;
	}
	
	// crossSegmentationThreshold /usr/include/opencv2/optflow/rlofflow.hpp:112
	void cv_optflow_RLOFOpticalFlowParameter_setPropCrossSegmentationThreshold_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
			instance->crossSegmentationThreshold = val;
	}
	
	// maxLevel /usr/include/opencv2/optflow/rlofflow.hpp:117
	int cv_optflow_RLOFOpticalFlowParameter_getPropMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->maxLevel;
			return ret;
	}
	
	// maxLevel /usr/include/opencv2/optflow/rlofflow.hpp:117
	void cv_optflow_RLOFOpticalFlowParameter_setPropMaxLevel_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
			instance->maxLevel = val;
	}
	
	// useInitialFlow /usr/include/opencv2/optflow/rlofflow.hpp:122
	bool cv_optflow_RLOFOpticalFlowParameter_getPropUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useInitialFlow;
			return ret;
	}
	
	// useInitialFlow /usr/include/opencv2/optflow/rlofflow.hpp:122
	void cv_optflow_RLOFOpticalFlowParameter_setPropUseInitialFlow_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
			instance->useInitialFlow = val;
	}
	
	// useIlluminationModel /usr/include/opencv2/optflow/rlofflow.hpp:126
	bool cv_optflow_RLOFOpticalFlowParameter_getPropUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useIlluminationModel;
			return ret;
	}
	
	// useIlluminationModel /usr/include/opencv2/optflow/rlofflow.hpp:126
	void cv_optflow_RLOFOpticalFlowParameter_setPropUseIlluminationModel_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
			instance->useIlluminationModel = val;
	}
	
	// useGlobalMotionPrior /usr/include/opencv2/optflow/rlofflow.hpp:134
	bool cv_optflow_RLOFOpticalFlowParameter_getPropUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			bool ret = instance->useGlobalMotionPrior;
			return ret;
	}
	
	// useGlobalMotionPrior /usr/include/opencv2/optflow/rlofflow.hpp:134
	void cv_optflow_RLOFOpticalFlowParameter_setPropUseGlobalMotionPrior_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val) {
			instance->useGlobalMotionPrior = val;
	}
	
	// maxIteration /usr/include/opencv2/optflow/rlofflow.hpp:139
	int cv_optflow_RLOFOpticalFlowParameter_getPropMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			int ret = instance->maxIteration;
			return ret;
	}
	
	// maxIteration /usr/include/opencv2/optflow/rlofflow.hpp:139
	void cv_optflow_RLOFOpticalFlowParameter_setPropMaxIteration_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val) {
			instance->maxIteration = val;
	}
	
	// minEigenValue /usr/include/opencv2/optflow/rlofflow.hpp:143
	float cv_optflow_RLOFOpticalFlowParameter_getPropMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->minEigenValue;
			return ret;
	}
	
	// minEigenValue /usr/include/opencv2/optflow/rlofflow.hpp:143
	void cv_optflow_RLOFOpticalFlowParameter_setPropMinEigenValue_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
			instance->minEigenValue = val;
	}
	
	// globalMotionRansacThreshold /usr/include/opencv2/optflow/rlofflow.hpp:147
	float cv_optflow_RLOFOpticalFlowParameter_getPropGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance) {
			float ret = instance->globalMotionRansacThreshold;
			return ret;
	}
	
	// globalMotionRansacThreshold /usr/include/opencv2/optflow/rlofflow.hpp:147
	void cv_optflow_RLOFOpticalFlowParameter_setPropGlobalMotionRansacThreshold_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val) {
			instance->globalMotionRansacThreshold = val;
	}
	
	void cv_RLOFOpticalFlowParameter_delete(cv::optflow::RLOFOpticalFlowParameter* instance) {
		delete instance;
	}
	// RLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:66
	void cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(Result<cv::optflow::RLOFOpticalFlowParameter*>* ocvrs_return) {
		try {
			cv::optflow::RLOFOpticalFlowParameter* ret = new cv::optflow::RLOFOpticalFlowParameter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::RLOFOpticalFlowParameter*>))
	}
	
	// setUseMEstimator(bool) /usr/include/opencv2/optflow/rlofflow.hpp:160
	void cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseMEstimator(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSolverType(cv::optflow::SolverType) /usr/include/opencv2/optflow/rlofflow.hpp:162
	void cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SolverType val, Result_void* ocvrs_return) {
		try {
			instance->setSolverType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSolverType() /usr/include/opencv2/optflow/rlofflow.hpp:163
	void cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<cv::optflow::SolverType>* ocvrs_return) {
		try {
			cv::optflow::SolverType ret = instance->getSolverType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SolverType>))
	}
	
	// setSupportRegionType(cv::optflow::SupportRegionType) /usr/include/opencv2/optflow/rlofflow.hpp:165
	void cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(cv::optflow::RLOFOpticalFlowParameter* instance, cv::optflow::SupportRegionType val, Result_void* ocvrs_return) {
		try {
			instance->setSupportRegionType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSupportRegionType() /usr/include/opencv2/optflow/rlofflow.hpp:166
	void cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<cv::optflow::SupportRegionType>* ocvrs_return) {
		try {
			cv::optflow::SupportRegionType ret = instance->getSupportRegionType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::optflow::SupportRegionType>))
	}
	
	// setNormSigma0(float) /usr/include/opencv2/optflow/rlofflow.hpp:168
	void cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setNormSigma0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormSigma0() /usr/include/opencv2/optflow/rlofflow.hpp:169
	void cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNormSigma0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setNormSigma1(float) /usr/include/opencv2/optflow/rlofflow.hpp:171
	void cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setNormSigma1(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormSigma1() /usr/include/opencv2/optflow/rlofflow.hpp:172
	void cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNormSigma1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSmallWinSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:174
	void cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setSmallWinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSmallWinSize() /usr/include/opencv2/optflow/rlofflow.hpp:175
	void cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmallWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setLargeWinSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:177
	void cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setLargeWinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLargeWinSize() /usr/include/opencv2/optflow/rlofflow.hpp:178
	void cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLargeWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCrossSegmentationThreshold(int) /usr/include/opencv2/optflow/rlofflow.hpp:180
	void cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setCrossSegmentationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCrossSegmentationThreshold() /usr/include/opencv2/optflow/rlofflow.hpp:181
	void cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCrossSegmentationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxLevel(int) /usr/include/opencv2/optflow/rlofflow.hpp:183
	void cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxLevel() /usr/include/opencv2/optflow/rlofflow.hpp:184
	void cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setUseInitialFlow(bool) /usr/include/opencv2/optflow/rlofflow.hpp:186
	void cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseInitialFlow() /usr/include/opencv2/optflow/rlofflow.hpp:187
	void cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseIlluminationModel(bool) /usr/include/opencv2/optflow/rlofflow.hpp:189
	void cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseIlluminationModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseIlluminationModel() /usr/include/opencv2/optflow/rlofflow.hpp:190
	void cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseIlluminationModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseGlobalMotionPrior(bool) /usr/include/opencv2/optflow/rlofflow.hpp:192
	void cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(cv::optflow::RLOFOpticalFlowParameter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseGlobalMotionPrior(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseGlobalMotionPrior() /usr/include/opencv2/optflow/rlofflow.hpp:193
	void cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseGlobalMotionPrior();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setMaxIteration(int) /usr/include/opencv2/optflow/rlofflow.hpp:195
	void cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(cv::optflow::RLOFOpticalFlowParameter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxIteration(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxIteration() /usr/include/opencv2/optflow/rlofflow.hpp:196
	void cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxIteration();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinEigenValue(float) /usr/include/opencv2/optflow/rlofflow.hpp:198
	void cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setMinEigenValue(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinEigenValue() /usr/include/opencv2/optflow/rlofflow.hpp:199
	void cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinEigenValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setGlobalMotionRansacThreshold(float) /usr/include/opencv2/optflow/rlofflow.hpp:201
	void cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(cv::optflow::RLOFOpticalFlowParameter* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setGlobalMotionRansacThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGlobalMotionRansacThreshold() /usr/include/opencv2/optflow/rlofflow.hpp:202
	void cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(const cv::optflow::RLOFOpticalFlowParameter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGlobalMotionRansacThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create() /usr/include/opencv2/optflow/rlofflow.hpp:205
	void cv_optflow_RLOFOpticalFlowParameter_create(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = cv::optflow::RLOFOpticalFlowParameter::create();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	// setRLOFOpticalFlowParameter(Ptr<cv::optflow::RLOFOpticalFlowParameter>) /usr/include/opencv2/optflow/rlofflow.hpp:417
	void cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(cv::optflow::SparseRLOFOpticalFlow* instance, cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* val, Result_void* ocvrs_return) {
		try {
			instance->setRLOFOpticalFlowParameter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:421
	void cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(const cv::optflow::SparseRLOFOpticalFlow* instance, Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::RLOFOpticalFlowParameter> ret = instance->getRLOFOpticalFlowParameter();
			Ok(new cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>))
	}
	
	// setForwardBackward(float) /usr/include/opencv2/optflow/rlofflow.hpp:430
	void cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(cv::optflow::SparseRLOFOpticalFlow* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setForwardBackward(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getForwardBackward() /usr/include/opencv2/optflow/rlofflow.hpp:434
	void cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(const cv::optflow::SparseRLOFOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getForwardBackward();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create(Ptr<cv::optflow::RLOFOpticalFlowParameter>, float) /usr/include/opencv2/optflow/rlofflow.hpp:441
	void cv_optflow_SparseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* rlofParam, float forwardBackwardThreshold, Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::optflow::SparseRLOFOpticalFlow> ret = cv::optflow::SparseRLOFOpticalFlow::create(*rlofParam, forwardBackwardThreshold);
			Ok(new cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>))
	}
	
}
