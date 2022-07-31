extern "C" {
	void cv_PtrOfLineSegmentDetector_delete(cv::Ptr<cv::LineSegmentDetector>* instance) {
		delete instance;
	}

	const cv::LineSegmentDetector* cv_PtrOfLineSegmentDetector_get_inner_ptr(const cv::Ptr<cv::LineSegmentDetector>* instance) {
		return instance->get();
	}

	cv::LineSegmentDetector* cv_PtrOfLineSegmentDetector_get_inner_ptr_mut(cv::Ptr<cv::LineSegmentDetector>* instance) {
		return instance->get();
	}
}

