extern "C" {
	void cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
		return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
}

