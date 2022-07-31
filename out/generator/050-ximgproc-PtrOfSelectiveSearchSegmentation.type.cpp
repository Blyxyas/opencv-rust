extern "C" {
	void cv_PtrOfSelectiveSearchSegmentation_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
		return instance->get();
	}
}

