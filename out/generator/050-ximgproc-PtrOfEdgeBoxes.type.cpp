extern "C" {
	void cv_PtrOfEdgeBoxes_delete(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
		delete instance;
	}

	const cv::ximgproc::EdgeBoxes* cv_PtrOfEdgeBoxes_get_inner_ptr(const cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
		return instance->get();
	}

	cv::ximgproc::EdgeBoxes* cv_PtrOfEdgeBoxes_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
		return instance->get();
	}
}

