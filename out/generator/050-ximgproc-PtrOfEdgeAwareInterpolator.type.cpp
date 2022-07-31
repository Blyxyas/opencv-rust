extern "C" {
	void cv_PtrOfEdgeAwareInterpolator_delete(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
		delete instance;
	}

	const cv::ximgproc::EdgeAwareInterpolator* cv_PtrOfEdgeAwareInterpolator_get_inner_ptr(const cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
		return instance->get();
	}

	cv::ximgproc::EdgeAwareInterpolator* cv_PtrOfEdgeAwareInterpolator_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
		return instance->get();
	}
}

