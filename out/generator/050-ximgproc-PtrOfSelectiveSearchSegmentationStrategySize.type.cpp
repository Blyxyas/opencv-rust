extern "C" {
	void cv_PtrOfSelectiveSearchSegmentationStrategySize_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrOfSelectiveSearchSegmentationStrategySize_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
		return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
}

