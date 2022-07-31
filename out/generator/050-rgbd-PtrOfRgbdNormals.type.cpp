extern "C" {
	cv::Ptr<cv::rgbd::RgbdNormals>* cv_PtrOfRgbdNormals_new(cv::rgbd::RgbdNormals* val) {
		return new cv::Ptr<cv::rgbd::RgbdNormals>(val);
	}
	
	void cv_PtrOfRgbdNormals_delete(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		delete instance;
	}

	const cv::rgbd::RgbdNormals* cv_PtrOfRgbdNormals_get_inner_ptr(const cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		return instance->get();
	}

	cv::rgbd::RgbdNormals* cv_PtrOfRgbdNormals_get_inner_ptr_mut(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
		return instance->get();
	}
}

