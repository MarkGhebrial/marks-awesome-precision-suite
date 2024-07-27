#include <opencv2/opencv.hpp>
#include <iostream>
#include <math.h>

void erodeThenDilate(cv::Mat inputImage, cv::Mat outputImage) {
    // Circle with diameter of 30
    auto kernel = cv::getStructuringElement(cv::MORPH_ELLIPSE, cv::Size(30, 30));

    cv::erode(inputImage, outputImage, kernel);
    cv::dilate(outputImage, outputImage, kernel);
}

double angleBetweenPoints(cv::Point p1, cv::Point corner, cv::Point p3) {
    //cv::Vec<cv::int, 2> v1 = cv::Vec((corner - p1).x, 0);

    cv::Point v1 = p1 - corner;
    cv::Point v2 = p3 - corner;

    double v1length = std::sqrt(v1.x*v1.x + v1.y*v1.y);
    double v2length = std::sqrt(v2.x*v2.x + v2.y*v2.y);

    return std::acos(v1.dot(v2)/(v1length*v2length));
}

// Plenary power
// Name one other beauro under the Dept. of the Interior