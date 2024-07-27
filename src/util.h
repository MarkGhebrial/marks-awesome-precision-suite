#include <opencv2/opencv.hpp>

void erodeThenDilate(cv::Mat inputImage, cv::Mat outputImage);

double angleBetweenPoints(cv::Point p1, cv::Point corner, cv::Point p3);