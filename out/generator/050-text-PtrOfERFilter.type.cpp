extern "C" {
	void cv_PtrOfERFilter_delete(cv::Ptr<cv::text::ERFilter>* instance) {
		delete instance;
	}

	const cv::text::ERFilter* cv_PtrOfERFilter_get_inner_ptr(const cv::Ptr<cv::text::ERFilter>* instance) {
		return instance->get();
	}

	cv::text::ERFilter* cv_PtrOfERFilter_get_inner_ptr_mut(cv::Ptr<cv::text::ERFilter>* instance) {
		return instance->get();
	}
}

