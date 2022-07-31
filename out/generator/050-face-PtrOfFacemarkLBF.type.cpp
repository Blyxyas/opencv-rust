extern "C" {
	void cv_PtrOfFacemarkLBF_delete(cv::Ptr<cv::face::FacemarkLBF>* instance) {
		delete instance;
	}

	const cv::face::FacemarkLBF* cv_PtrOfFacemarkLBF_get_inner_ptr(const cv::Ptr<cv::face::FacemarkLBF>* instance) {
		return instance->get();
	}

	cv::face::FacemarkLBF* cv_PtrOfFacemarkLBF_get_inner_ptr_mut(cv::Ptr<cv::face::FacemarkLBF>* instance) {
		return instance->get();
	}
}

