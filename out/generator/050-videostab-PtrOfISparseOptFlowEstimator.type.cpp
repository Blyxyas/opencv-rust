extern "C" {
	void cv_PtrOfISparseOptFlowEstimator_delete(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::ISparseOptFlowEstimator* cv_PtrOfISparseOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::ISparseOptFlowEstimator* cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* instance) {
		return instance->get();
	}
}

