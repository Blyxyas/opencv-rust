extern "C" {
	void cv_PtrOfERFilter_Callback_delete(cv::Ptr<cv::text::ERFilter::Callback>* instance) {
		delete instance;
	}

	const cv::text::ERFilter::Callback* cv_PtrOfERFilter_Callback_get_inner_ptr(const cv::Ptr<cv::text::ERFilter::Callback>* instance) {
		return instance->get();
	}

	cv::text::ERFilter::Callback* cv_PtrOfERFilter_Callback_get_inner_ptr_mut(cv::Ptr<cv::text::ERFilter::Callback>* instance) {
		return instance->get();
	}
}

