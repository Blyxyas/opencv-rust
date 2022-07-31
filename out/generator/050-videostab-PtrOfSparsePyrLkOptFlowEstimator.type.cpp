extern "C" {
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* cv_PtrOfSparsePyrLkOptFlowEstimator_new(cv::videostab::SparsePyrLkOptFlowEstimator* val) {
		return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>(val);
	}
	
	void cv_PtrOfSparsePyrLkOptFlowEstimator_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrOfSparsePyrLkOptFlowEstimator_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
		return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}
}

