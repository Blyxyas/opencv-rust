#include "ocvrs_common.hpp"
#include <opencv2/phase_unwrapping.hpp>
#include "phase_unwrapping_types.hpp"

extern "C" {
	// create(const HistogramPhaseUnwrapping::Params &) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:93
	void cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(const cv::phase_unwrapping::HistogramPhaseUnwrapping::Params* parameters, Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping> ret = cv::phase_unwrapping::HistogramPhaseUnwrapping::create(*parameters);
			Ok(new cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*>))
	}
	
	// getInverseReliabilityMap(cv::OutputArray) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:102
	void cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(cv::phase_unwrapping::HistogramPhaseUnwrapping* instance, const cv::_OutputArray* reliabilityMap, Result_void* ocvrs_return) {
		try {
			instance->getInverseReliabilityMap(*reliabilityMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Params() /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:80
	void cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params(Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params>* ocvrs_return) {
		try {
			cv::phase_unwrapping::HistogramPhaseUnwrapping::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::phase_unwrapping::HistogramPhaseUnwrapping::Params>))
	}
	
	// unwrapPhaseMap(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/phase_unwrapping/phase_unwrapping.hpp:66
	void cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::phase_unwrapping::PhaseUnwrapping* instance, const cv::_InputArray* wrappedPhaseMap, const cv::_OutputArray* unwrappedPhaseMap, const cv::_InputArray* shadowMask, Result_void* ocvrs_return) {
		try {
			instance->unwrapPhaseMap(*wrappedPhaseMap, *unwrappedPhaseMap, *shadowMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
