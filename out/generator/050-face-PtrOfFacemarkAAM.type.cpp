extern "C" {
	void cv_PtrOfFacemarkAAM_delete(cv::Ptr<cv::face::FacemarkAAM>* instance) {
		delete instance;
	}

	const cv::face::FacemarkAAM* cv_PtrOfFacemarkAAM_get_inner_ptr(const cv::Ptr<cv::face::FacemarkAAM>* instance) {
		return instance->get();
	}

	cv::face::FacemarkAAM* cv_PtrOfFacemarkAAM_get_inner_ptr_mut(cv::Ptr<cv::face::FacemarkAAM>* instance) {
		return instance->get();
	}
}

