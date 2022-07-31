extern "C" {
	void cv_PtrOfSinusoidalPattern_delete(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
		delete instance;
	}

	const cv::structured_light::SinusoidalPattern* cv_PtrOfSinusoidalPattern_get_inner_ptr(const cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
		return instance->get();
	}

	cv::structured_light::SinusoidalPattern* cv_PtrOfSinusoidalPattern_get_inner_ptr_mut(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
		return instance->get();
	}
}

