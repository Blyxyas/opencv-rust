extern "C" {
	void cv_PtrOfIDenseOptFlowEstimator_delete(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::IDenseOptFlowEstimator* cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::IDenseOptFlowEstimator* cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* instance) {
		return instance->get();
	}
}

