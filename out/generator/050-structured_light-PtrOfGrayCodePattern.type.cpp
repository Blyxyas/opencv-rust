extern "C" {
	void cv_PtrOfGrayCodePattern_delete(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
		delete instance;
	}

	const cv::structured_light::GrayCodePattern* cv_PtrOfGrayCodePattern_get_inner_ptr(const cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
		return instance->get();
	}

	cv::structured_light::GrayCodePattern* cv_PtrOfGrayCodePattern_get_inner_ptr_mut(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
		return instance->get();
	}
}

