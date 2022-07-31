extern "C" {
	cv::Ptr<cv::videostab::InpaintingPipeline>* cv_PtrOfInpaintingPipeline_new(cv::videostab::InpaintingPipeline* val) {
		return new cv::Ptr<cv::videostab::InpaintingPipeline>(val);
	}
	
	void cv_PtrOfInpaintingPipeline_delete(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		delete instance;
	}

	const cv::videostab::InpaintingPipeline* cv_PtrOfInpaintingPipeline_get_inner_ptr(const cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return instance->get();
	}

	cv::videostab::InpaintingPipeline* cv_PtrOfInpaintingPipeline_get_inner_ptr_mut(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfInpaintingPipeline_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

