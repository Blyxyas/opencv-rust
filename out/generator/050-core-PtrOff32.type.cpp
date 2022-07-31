extern "C" {
	cv::Ptr<float>* cv_PtrOff32_new(float val) {
		return new cv::Ptr<float>(new float(val));
	}
	
	void cv_PtrOff32_delete(cv::Ptr<float>* instance) {
		delete instance;
	}

	const float* cv_PtrOff32_get_inner_ptr(const cv::Ptr<float>* instance) {
		return instance->get();
	}

	float* cv_PtrOff32_get_inner_ptr_mut(cv::Ptr<float>* instance) {
		return instance->get();
	}
}

