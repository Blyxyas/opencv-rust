extern "C" {
	void cv_PtrOfFastBilateralSolverFilter_delete(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
		delete instance;
	}

	const cv::ximgproc::FastBilateralSolverFilter* cv_PtrOfFastBilateralSolverFilter_get_inner_ptr(const cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
		return instance->get();
	}

	cv::ximgproc::FastBilateralSolverFilter* cv_PtrOfFastBilateralSolverFilter_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
		return instance->get();
	}
}

