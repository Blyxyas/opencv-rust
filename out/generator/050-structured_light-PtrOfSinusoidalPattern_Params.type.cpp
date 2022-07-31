extern "C" {
	cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* cv_PtrOfSinusoidalPattern_Params_new(cv::structured_light::SinusoidalPattern::Params* val) {
		return new cv::Ptr<cv::structured_light::SinusoidalPattern::Params>(val);
	}
	
	void cv_PtrOfSinusoidalPattern_Params_delete(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* instance) {
		delete instance;
	}

	const cv::structured_light::SinusoidalPattern::Params* cv_PtrOfSinusoidalPattern_Params_get_inner_ptr(const cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* instance) {
		return instance->get();
	}

	cv::structured_light::SinusoidalPattern::Params* cv_PtrOfSinusoidalPattern_Params_get_inner_ptr_mut(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* instance) {
		return instance->get();
	}
}

