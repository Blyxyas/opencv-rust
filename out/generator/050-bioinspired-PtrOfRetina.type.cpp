extern "C" {
	void cv_PtrOfRetina_delete(cv::Ptr<cv::bioinspired::Retina>* instance) {
		delete instance;
	}

	const cv::bioinspired::Retina* cv_PtrOfRetina_get_inner_ptr(const cv::Ptr<cv::bioinspired::Retina>* instance) {
		return instance->get();
	}

	cv::bioinspired::Retina* cv_PtrOfRetina_get_inner_ptr_mut(cv::Ptr<cv::bioinspired::Retina>* instance) {
		return instance->get();
	}
}

