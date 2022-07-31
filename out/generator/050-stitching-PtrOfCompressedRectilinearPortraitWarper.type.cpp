extern "C" {
	cv::Ptr<cv::CompressedRectilinearPortraitWarper>* cv_PtrOfCompressedRectilinearPortraitWarper_new(cv::CompressedRectilinearPortraitWarper* val) {
		return new cv::Ptr<cv::CompressedRectilinearPortraitWarper>(val);
	}
	
	void cv_PtrOfCompressedRectilinearPortraitWarper_delete(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		delete instance;
	}

	const cv::CompressedRectilinearPortraitWarper* cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr(const cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return instance->get();
	}

	cv::CompressedRectilinearPortraitWarper* cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

