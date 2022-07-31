extern "C" {
	void cv_PtrOfConjGradSolver_delete(cv::Ptr<cv::ConjGradSolver>* instance) {
		delete instance;
	}

	const cv::ConjGradSolver* cv_PtrOfConjGradSolver_get_inner_ptr(const cv::Ptr<cv::ConjGradSolver>* instance) {
		return instance->get();
	}

	cv::ConjGradSolver* cv_PtrOfConjGradSolver_get_inner_ptr_mut(cv::Ptr<cv::ConjGradSolver>* instance) {
		return instance->get();
	}
}

