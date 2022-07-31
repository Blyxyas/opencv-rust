extern "C" {
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrOfRgbdFrame_new(cv::rgbd::RgbdFrame* val) {
		return new cv::Ptr<cv::rgbd::RgbdFrame>(val);
	}
	
	void cv_PtrOfRgbdFrame_delete(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdFrame* cv_PtrOfRgbdFrame_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdFrame* cv_PtrOfRgbdFrame_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
		return instance->get();
	}
}

