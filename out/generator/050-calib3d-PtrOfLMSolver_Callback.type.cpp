extern "C" {
	void cv_PtrOfLMSolver_Callback_delete(cv::Ptr<cv::LMSolver::Callback>* instance) {
		delete instance;
	}

	const cv::LMSolver::Callback* cv_PtrOfLMSolver_Callback_get_inner_ptr(const cv::Ptr<cv::LMSolver::Callback>* instance) {
		return instance->get();
	}

	cv::LMSolver::Callback* cv_PtrOfLMSolver_Callback_get_inner_ptr_mut(cv::Ptr<cv::LMSolver::Callback>* instance) {
		return instance->get();
	}
}

