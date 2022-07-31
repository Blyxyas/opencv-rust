extern "C" {
	void cv_PtrOfFacemarkKazemi_delete(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
		delete instance;
	}

	const cv::face::FacemarkKazemi* cv_PtrOfFacemarkKazemi_get_inner_ptr(const cv::Ptr<cv::face::FacemarkKazemi>* instance) {
		return instance->get();
	}

	cv::face::FacemarkKazemi* cv_PtrOfFacemarkKazemi_get_inner_ptr_mut(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
		return instance->get();
	}
}

