extern "C" {
	void cv_PtrOfDownhillSolver_delete(cv::Ptr<cv::DownhillSolver>* instance) {
		delete instance;
	}

	const cv::DownhillSolver* cv_PtrOfDownhillSolver_get_inner_ptr(const cv::Ptr<cv::DownhillSolver>* instance) {
		return instance->get();
	}

	cv::DownhillSolver* cv_PtrOfDownhillSolver_get_inner_ptr_mut(cv::Ptr<cv::DownhillSolver>* instance) {
		return instance->get();
	}
}

