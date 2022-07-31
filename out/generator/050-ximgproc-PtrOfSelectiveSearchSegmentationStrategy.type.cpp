extern "C" {
	void cv_PtrOfSelectiveSearchSegmentationStrategy_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
		return instance->get();
	}
}

