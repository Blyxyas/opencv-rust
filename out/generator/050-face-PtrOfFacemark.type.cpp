extern "C" {
	void cv_PtrOfFacemark_delete(cv::Ptr<cv::face::Facemark>* instance) {
		delete instance;
	}

	const cv::face::Facemark* cv_PtrOfFacemark_get_inner_ptr(const cv::Ptr<cv::face::Facemark>* instance) {
		return instance->get();
	}

	cv::face::Facemark* cv_PtrOfFacemark_get_inner_ptr_mut(cv::Ptr<cv::face::Facemark>* instance) {
		return instance->get();
	}
}

