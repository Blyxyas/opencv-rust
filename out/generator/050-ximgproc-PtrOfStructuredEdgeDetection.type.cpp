extern "C" {
	void cv_PtrOfStructuredEdgeDetection_delete(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
		delete instance;
	}

	const cv::ximgproc::StructuredEdgeDetection* cv_PtrOfStructuredEdgeDetection_get_inner_ptr(const cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
		return instance->get();
	}

	cv::ximgproc::StructuredEdgeDetection* cv_PtrOfStructuredEdgeDetection_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
		return instance->get();
	}
}

