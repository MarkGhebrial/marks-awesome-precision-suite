#include <opencv2/opencv.hpp>

void erodeThenDilate(const cv::Mat& inputImage, cv::Mat outputImage, int size);
void contrast(const cv::Mat& inputImage, cv::Mat outputImage, double alpha, double beta);

double angleBetweenPoints(cv::Point p1, cv::Point corner, cv::Point p3);