extern "C" {
	cv::Ptr<cv::kinfu::VolumeParams>* cv_PtrOfKinfu_VolumeParams_new(cv::kinfu::VolumeParams* val) {
		return new cv::Ptr<cv::kinfu::VolumeParams>(val);
	}
	
	void cv_PtrOfKinfu_VolumeParams_delete(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
		delete instance;
	}

	const cv::kinfu::VolumeParams* cv_PtrOfKinfu_VolumeParams_get_inner_ptr(const cv::Ptr<cv::kinfu::VolumeParams>* instance) {
		return instance->get();
	}

	cv::kinfu::VolumeParams* cv_PtrOfKinfu_VolumeParams_get_inner_ptr_mut(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
		return instance->get();
	}
}

