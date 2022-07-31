extern "C" {
	cv::Ptr<cv::line_descriptor::LSDDetector>* cv_PtrOfLSDDetector_new(cv::line_descriptor::LSDDetector* val) {
		return new cv::Ptr<cv::line_descriptor::LSDDetector>(val);
	}
	
	void cv_PtrOfLSDDetector_delete(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
		delete instance;
	}

	const cv::line_descriptor::LSDDetector* cv_PtrOfLSDDetector_get_inner_ptr(const cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
		return instance->get();
	}

	cv::line_descriptor::LSDDetector* cv_PtrOfLSDDetector_get_inner_ptr_mut(cv::Ptr<cv::line_descriptor::LSDDetector>* instance) {
		return instance->get();
	}
}

