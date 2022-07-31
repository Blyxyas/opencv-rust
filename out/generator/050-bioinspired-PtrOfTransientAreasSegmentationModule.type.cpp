extern "C" {
	void cv_PtrOfTransientAreasSegmentationModule_delete(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
		delete instance;
	}

	const cv::bioinspired::TransientAreasSegmentationModule* cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr(const cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
		return instance->get();
	}

	cv::bioinspired::TransientAreasSegmentationModule* cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr_mut(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
		return instance->get();
	}
}

