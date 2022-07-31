extern "C" {
	void cv_PtrOfFaceRecognizerSF_delete(cv::Ptr<cv::FaceRecognizerSF>* instance) {
		delete instance;
	}

	const cv::FaceRecognizerSF* cv_PtrOfFaceRecognizerSF_get_inner_ptr(const cv::Ptr<cv::FaceRecognizerSF>* instance) {
		return instance->get();
	}

	cv::FaceRecognizerSF* cv_PtrOfFaceRecognizerSF_get_inner_ptr_mut(cv::Ptr<cv::FaceRecognizerSF>* instance) {
		return instance->get();
	}
}

