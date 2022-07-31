extern "C" {
	void cv_PtrOfTextDetectorCNN_delete(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
		delete instance;
	}

	const cv::text::TextDetectorCNN* cv_PtrOfTextDetectorCNN_get_inner_ptr(const cv::Ptr<cv::text::TextDetectorCNN>* instance) {
		return instance->get();
	}

	cv::text::TextDetectorCNN* cv_PtrOfTextDetectorCNN_get_inner_ptr_mut(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
		return instance->get();
	}
}

