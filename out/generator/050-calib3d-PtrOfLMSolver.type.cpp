extern "C" {
	void cv_PtrOfLMSolver_delete(cv::Ptr<cv::LMSolver>* instance) {
		delete instance;
	}

	const cv::LMSolver* cv_PtrOfLMSolver_get_inner_ptr(const cv::Ptr<cv::LMSolver>* instance) {
		return instance->get();
	}

	cv::LMSolver* cv_PtrOfLMSolver_get_inner_ptr_mut(cv::Ptr<cv::LMSolver>* instance) {
		return instance->get();
	}
}

