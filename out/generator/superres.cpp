#include "ocvrs_common.hpp"
#include <opencv2/superres.hpp>
#include "superres_types.hpp"

extern "C" {
	// createFrameSource_Camera(int) /usr/include/opencv2/superres.hpp:80
	void cv_superres_createFrameSource_Camera_int(int deviceId, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Camera(deviceId);
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FrameSource>*>))
	}
	
	// createFrameSource_Empty() /usr/include/opencv2/superres.hpp:75
	void cv_superres_createFrameSource_Empty(Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Empty();
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FrameSource>*>))
	}
	
	// createFrameSource_Video_CUDA(const cv::String &) /usr/include/opencv2/superres.hpp:78
	void cv_superres_createFrameSource_Video_CUDA_const_StringR(const char* fileName, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video_CUDA(std::string(fileName));
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FrameSource>*>))
	}
	
	// createFrameSource_Video(const cv::String &) /usr/include/opencv2/superres.hpp:77
	void cv_superres_createFrameSource_Video_const_StringR(const char* fileName, Result<cv::Ptr<cv::superres::FrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FrameSource> ret = cv::superres::createFrameSource_Video(std::string(fileName));
			Ok(new cv::Ptr<cv::superres::FrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FrameSource>*>))
	}
	
	// createOptFlow_Brox_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:177
	void cv_superres_createOptFlow_Brox_CUDA(Result<cv::Ptr<cv::superres::BroxOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::BroxOpticalFlow> ret = cv::superres::createOptFlow_Brox_CUDA();
			Ok(new cv::Ptr<cv::superres::BroxOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::BroxOpticalFlow>*>))
	}
	
	// createOptFlow_DualTVL1() /usr/include/opencv2/superres/optical_flow.hpp:139
	void cv_superres_createOptFlow_DualTVL1(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1();
			Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>))
	}
	
	// createOptFlow_DualTVL1_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:140
	void cv_superres_createOptFlow_DualTVL1_CUDA(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DualTVL1OpticalFlow> ret = cv::superres::createOptFlow_DualTVL1_CUDA();
			Ok(new cv::Ptr<cv::superres::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::DualTVL1OpticalFlow>*>))
	}
	
	// createOptFlow_Farneback() /usr/include/opencv2/superres/optical_flow.hpp:96
	void cv_superres_createOptFlow_Farneback(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback();
			Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>))
	}
	
	// createOptFlow_Farneback_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:97
	void cv_superres_createOptFlow_Farneback_CUDA(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::FarnebackOpticalFlow> ret = cv::superres::createOptFlow_Farneback_CUDA();
			Ok(new cv::Ptr<cv::superres::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::FarnebackOpticalFlow>*>))
	}
	
	// createOptFlow_PyrLK_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:196
	void cv_superres_createOptFlow_PyrLK_CUDA(Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::PyrLKOpticalFlow> ret = cv::superres::createOptFlow_PyrLK_CUDA();
			Ok(new cv::Ptr<cv::superres::PyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::PyrLKOpticalFlow>*>))
	}
	
	// createSuperResolution_BTVL1() /usr/include/opencv2/superres.hpp:199
	void cv_superres_createSuperResolution_BTVL1(Result<cv::Ptr<cv::superres::SuperResolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1();
			Ok(new cv::Ptr<cv::superres::SuperResolution>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::SuperResolution>*>))
	}
	
	// createSuperResolution_BTVL1_CUDA() /usr/include/opencv2/superres.hpp:200
	void cv_superres_createSuperResolution_BTVL1_CUDA(Result<cv::Ptr<cv::superres::SuperResolution>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::SuperResolution> ret = cv::superres::createSuperResolution_BTVL1_CUDA();
			Ok(new cv::Ptr<cv::superres::SuperResolution>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::SuperResolution>*>))
	}
	
	// getAlpha() /usr/include/opencv2/superres/optical_flow.hpp:148
	void cv_superres_BroxOpticalFlow_getAlpha_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setAlpha(double) /usr/include/opencv2/superres/optical_flow.hpp:150
	void cv_superres_BroxOpticalFlow_setAlpha_double(cv::superres::BroxOpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGamma() /usr/include/opencv2/superres/optical_flow.hpp:153
	void cv_superres_BroxOpticalFlow_getGamma_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setGamma(double) /usr/include/opencv2/superres/optical_flow.hpp:155
	void cv_superres_BroxOpticalFlow_setGamma_double(cv::superres::BroxOpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScaleFactor() /usr/include/opencv2/superres/optical_flow.hpp:158
	void cv_superres_BroxOpticalFlow_getScaleFactor_const(const cv::superres::BroxOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setScaleFactor(double) /usr/include/opencv2/superres/optical_flow.hpp:160
	void cv_superres_BroxOpticalFlow_setScaleFactor_double(cv::superres::BroxOpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setScaleFactor(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInnerIterations() /usr/include/opencv2/superres/optical_flow.hpp:163
	void cv_superres_BroxOpticalFlow_getInnerIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setInnerIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:165
	void cv_superres_BroxOpticalFlow_setInnerIterations_int(cv::superres::BroxOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setInnerIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOuterIterations() /usr/include/opencv2/superres/optical_flow.hpp:168
	void cv_superres_BroxOpticalFlow_getOuterIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setOuterIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:170
	void cv_superres_BroxOpticalFlow_setOuterIterations_int(cv::superres::BroxOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setOuterIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSolverIterations() /usr/include/opencv2/superres/optical_flow.hpp:173
	void cv_superres_BroxOpticalFlow_getSolverIterations_const(const cv::superres::BroxOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSolverIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSolverIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:175
	void cv_superres_BroxOpticalFlow_setSolverIterations_int(cv::superres::BroxOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setSolverIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// calc(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/superres/optical_flow.hpp:59
	void cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::superres::DenseOpticalFlowExt* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_OutputArray* flow1, const cv::_OutputArray* flow2, Result_void* ocvrs_return) {
		try {
			instance->calc(*frame0, *frame1, *flow1, *flow2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// collectGarbage() /usr/include/opencv2/superres/optical_flow.hpp:60
	void cv_superres_DenseOpticalFlowExt_collectGarbage(cv::superres::DenseOpticalFlowExt* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTau() /usr/include/opencv2/superres/optical_flow.hpp:107
	void cv_superres_DualTVL1OpticalFlow_getTau_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setTau(double) /usr/include/opencv2/superres/optical_flow.hpp:109
	void cv_superres_DualTVL1OpticalFlow_setTau_double(cv::superres::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLambda() /usr/include/opencv2/superres/optical_flow.hpp:111
	void cv_superres_DualTVL1OpticalFlow_getLambda_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setLambda(double) /usr/include/opencv2/superres/optical_flow.hpp:113
	void cv_superres_DualTVL1OpticalFlow_setLambda_double(cv::superres::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTheta() /usr/include/opencv2/superres/optical_flow.hpp:115
	void cv_superres_DualTVL1OpticalFlow_getTheta_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setTheta(double) /usr/include/opencv2/superres/optical_flow.hpp:117
	void cv_superres_DualTVL1OpticalFlow_setTheta_double(cv::superres::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setTheta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScalesNumber() /usr/include/opencv2/superres/optical_flow.hpp:119
	void cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScalesNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setScalesNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:121
	void cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setScalesNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWarpingsNumber() /usr/include/opencv2/superres/optical_flow.hpp:123
	void cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWarpingsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWarpingsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:125
	void cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::superres::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWarpingsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEpsilon() /usr/include/opencv2/superres/optical_flow.hpp:127
	void cv_superres_DualTVL1OpticalFlow_getEpsilon_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setEpsilon(double) /usr/include/opencv2/superres/optical_flow.hpp:129
	void cv_superres_DualTVL1OpticalFlow_setEpsilon_double(cv::superres::DualTVL1OpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:131
	void cv_superres_DualTVL1OpticalFlow_getIterations_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:133
	void cv_superres_DualTVL1OpticalFlow_setIterations_int(cv::superres::DualTVL1OpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseInitialFlow() /usr/include/opencv2/superres/optical_flow.hpp:135
	void cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::superres::DualTVL1OpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseInitialFlow(bool) /usr/include/opencv2/superres/optical_flow.hpp:137
	void cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::superres::DualTVL1OpticalFlow* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPyrScale() /usr/include/opencv2/superres/optical_flow.hpp:68
	void cv_superres_FarnebackOpticalFlow_getPyrScale_const(const cv::superres::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setPyrScale(double) /usr/include/opencv2/superres/optical_flow.hpp:70
	void cv_superres_FarnebackOpticalFlow_setPyrScale_double(cv::superres::FarnebackOpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setPyrScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLevelsNumber() /usr/include/opencv2/superres/optical_flow.hpp:72
	void cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevelsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setLevelsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:74
	void cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(cv::superres::FarnebackOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setLevelsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:76
	void cv_superres_FarnebackOpticalFlow_getWindowSize_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:78
	void cv_superres_FarnebackOpticalFlow_setWindowSize_int(cv::superres::FarnebackOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:80
	void cv_superres_FarnebackOpticalFlow_getIterations_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:82
	void cv_superres_FarnebackOpticalFlow_setIterations_int(cv::superres::FarnebackOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPolyN() /usr/include/opencv2/superres/optical_flow.hpp:84
	void cv_superres_FarnebackOpticalFlow_getPolyN_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPolyN(int) /usr/include/opencv2/superres/optical_flow.hpp:86
	void cv_superres_FarnebackOpticalFlow_setPolyN_int(cv::superres::FarnebackOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setPolyN(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPolySigma() /usr/include/opencv2/superres/optical_flow.hpp:88
	void cv_superres_FarnebackOpticalFlow_getPolySigma_const(const cv::superres::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setPolySigma(double) /usr/include/opencv2/superres/optical_flow.hpp:90
	void cv_superres_FarnebackOpticalFlow_setPolySigma_double(cv::superres::FarnebackOpticalFlow* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setPolySigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFlags() /usr/include/opencv2/superres/optical_flow.hpp:92
	void cv_superres_FarnebackOpticalFlow_getFlags_const(const cv::superres::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setFlags(int) /usr/include/opencv2/superres/optical_flow.hpp:94
	void cv_superres_FarnebackOpticalFlow_setFlags_int(cv::superres::FarnebackOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setFlags(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:71
	void cv_superres_FrameSource_nextFrame_const__OutputArrayR(cv::superres::FrameSource* instance, const cv::_OutputArray* frame, Result_void* ocvrs_return) {
		try {
			instance->nextFrame(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/superres.hpp:72
	void cv_superres_FrameSource_reset(cv::superres::FrameSource* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:184
	void cv_superres_PyrLKOpticalFlow_getWindowSize_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:186
	void cv_superres_PyrLKOpticalFlow_setWindowSize_int(cv::superres::PyrLKOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxLevel() /usr/include/opencv2/superres/optical_flow.hpp:188
	void cv_superres_PyrLKOpticalFlow_getMaxLevel_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxLevel(int) /usr/include/opencv2/superres/optical_flow.hpp:190
	void cv_superres_PyrLKOpticalFlow_setMaxLevel_int(cv::superres::PyrLKOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:192
	void cv_superres_PyrLKOpticalFlow_getIterations_const(const cv::superres::PyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:194
	void cv_superres_PyrLKOpticalFlow_setIterations_int(cv::superres::PyrLKOpticalFlow* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInput(const Ptr<cv::superres::FrameSource> &) /usr/include/opencv2/superres.hpp:94
	void cv_superres_SuperResolution_setInput_const_Ptr_FrameSource_R(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::FrameSource>* frameSource, Result_void* ocvrs_return) {
		try {
			instance->setInput(*frameSource);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:100
	void cv_superres_SuperResolution_nextFrame_const__OutputArrayR(cv::superres::SuperResolution* instance, const cv::_OutputArray* frame, Result_void* ocvrs_return) {
		try {
			instance->nextFrame(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/superres.hpp:101
	void cv_superres_SuperResolution_reset(cv::superres::SuperResolution* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// collectGarbage() /usr/include/opencv2/superres.hpp:105
	void cv_superres_SuperResolution_collectGarbage(cv::superres::SuperResolution* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScale() /usr/include/opencv2/superres.hpp:109
	void cv_superres_SuperResolution_getScale_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setScale(int) /usr/include/opencv2/superres.hpp:111
	void cv_superres_SuperResolution_setScale_int(cv::superres::SuperResolution* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/superres.hpp:115
	void cv_superres_SuperResolution_getIterations_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setIterations(int) /usr/include/opencv2/superres.hpp:117
	void cv_superres_SuperResolution_setIterations_int(cv::superres::SuperResolution* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTau() /usr/include/opencv2/superres.hpp:121
	void cv_superres_SuperResolution_getTau_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setTau(double) /usr/include/opencv2/superres.hpp:123
	void cv_superres_SuperResolution_setTau_double(cv::superres::SuperResolution* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLambda() /usr/include/opencv2/superres.hpp:127
	void cv_superres_SuperResolution_getLambda_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setLambda(double) /usr/include/opencv2/superres.hpp:129
	void cv_superres_SuperResolution_setLambda_double(cv::superres::SuperResolution* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAlpha() /usr/include/opencv2/superres.hpp:133
	void cv_superres_SuperResolution_getAlpha_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setAlpha(double) /usr/include/opencv2/superres.hpp:135
	void cv_superres_SuperResolution_setAlpha_double(cv::superres::SuperResolution* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getKernelSize() /usr/include/opencv2/superres.hpp:139
	void cv_superres_SuperResolution_getKernelSize_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setKernelSize(int) /usr/include/opencv2/superres.hpp:141
	void cv_superres_SuperResolution_setKernelSize_int(cv::superres::SuperResolution* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setKernelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBlurKernelSize() /usr/include/opencv2/superres.hpp:145
	void cv_superres_SuperResolution_getBlurKernelSize_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlurKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setBlurKernelSize(int) /usr/include/opencv2/superres.hpp:147
	void cv_superres_SuperResolution_setBlurKernelSize_int(cv::superres::SuperResolution* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setBlurKernelSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBlurSigma() /usr/include/opencv2/superres.hpp:151
	void cv_superres_SuperResolution_getBlurSigma_const(const cv::superres::SuperResolution* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBlurSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBlurSigma(double) /usr/include/opencv2/superres.hpp:153
	void cv_superres_SuperResolution_setBlurSigma_double(cv::superres::SuperResolution* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setBlurSigma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTemporalAreaRadius() /usr/include/opencv2/superres.hpp:157
	void cv_superres_SuperResolution_getTemporalAreaRadius_const(const cv::superres::SuperResolution* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTemporalAreaRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTemporalAreaRadius(int) /usr/include/opencv2/superres.hpp:159
	void cv_superres_SuperResolution_setTemporalAreaRadius_int(cv::superres::SuperResolution* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTemporalAreaRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOpticalFlow() /usr/include/opencv2/superres.hpp:163
	void cv_superres_SuperResolution_getOpticalFlow_const(const cv::superres::SuperResolution* instance, Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::superres::DenseOpticalFlowExt> ret = instance->getOpticalFlow();
			Ok(new cv::Ptr<cv::superres::DenseOpticalFlowExt>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::superres::DenseOpticalFlowExt>*>))
	}
	
	// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &) /usr/include/opencv2/superres.hpp:165
	void cv_superres_SuperResolution_setOpticalFlow_const_Ptr_DenseOpticalFlowExt_R(cv::superres::SuperResolution* instance, const cv::Ptr<cv::superres::DenseOpticalFlowExt>* val, Result_void* ocvrs_return) {
		try {
			instance->setOpticalFlow(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
