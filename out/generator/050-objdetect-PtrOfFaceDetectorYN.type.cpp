extern "C" {
	void cv_PtrOfFaceDetectorYN_delete(cv::Ptr<cv::FaceDetectorYN>* instance) {
		delete instance;
	}

	const cv::FaceDetectorYN* cv_PtrOfFaceDetectorYN_get_inner_ptr(const cv::Ptr<cv::FaceDetectorYN>* instance) {
		return instance->get();
	}

	cv::FaceDetectorYN* cv_PtrOfFaceDetectorYN_get_inner_ptr_mut(cv::Ptr<cv::FaceDetectorYN>* instance) {
		return instance->get();
	}
}

