extern "C" {
	void cv_PtrOfEdgeDrawing_delete(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
		delete instance;
	}

	const cv::ximgproc::EdgeDrawing* cv_PtrOfEdgeDrawing_get_inner_ptr(const cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
		return instance->get();
	}

	cv::ximgproc::EdgeDrawing* cv_PtrOfEdgeDrawing_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
		return instance->get();
	}
}

