extern "C" {
	void cv_PtrOfMinProblemSolver_Function_delete(cv::Ptr<cv::MinProblemSolver::Function>* instance) {
		delete instance;
	}

	const cv::MinProblemSolver::Function* cv_PtrOfMinProblemSolver_Function_get_inner_ptr(const cv::Ptr<cv::MinProblemSolver::Function>* instance) {
		return instance->get();
	}

	cv::MinProblemSolver::Function* cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut(cv::Ptr<cv::MinProblemSolver::Function>* instance) {
		return instance->get();
	}
}

