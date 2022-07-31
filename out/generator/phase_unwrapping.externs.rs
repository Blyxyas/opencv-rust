extern "C" {
	// create(const HistogramPhaseUnwrapping::Params &) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:93
	pub fn cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(parameters: *const crate::phase_unwrapping::HistogramPhaseUnwrapping_Params, ocvrs_return: *mut Result<*mut c_void>);
	// getInverseReliabilityMap(cv::OutputArray) /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:102
	pub fn cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(instance: *mut c_void, reliability_map: *const c_void, ocvrs_return: *mut Result_void);
	// Params() /usr/include/opencv2/phase_unwrapping/histogramphaseunwrapping.hpp:80
	pub fn cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params(ocvrs_return: *mut Result<crate::phase_unwrapping::HistogramPhaseUnwrapping_Params>);
	// unwrapPhaseMap(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/phase_unwrapping/phase_unwrapping.hpp:66
	pub fn cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, wrapped_phase_map: *const c_void, unwrapped_phase_map: *const c_void, shadow_mask: *const c_void, ocvrs_return: *mut Result_void);
}
