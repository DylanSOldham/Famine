#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include <iostream>

#include "include/window.h"

class WindowImpl
{
public:
    WindowImpl(GLFWwindow* glfw_window) : glfw_window(glfw_window)
    {}

    GLFWwindow* glfw_window;
};

void error_callback(int code, const char* description)
{
    std::cerr << "GLFW Error: " << description << "\n";
}


extern "C"
{
    WindowImpl* window_create(const char* title, size_t width, size_t height) 
    {
        if (!glfwInit())
        {
            std::cerr << "GLFW Failed to Init\n";
            return nullptr;
        }

        glfwSetErrorCallback(error_callback);

        GLFWwindow* glfw_window = glfwCreateWindow(width, height, title, NULL, NULL);
        if (!glfw_window)
        {
            std::cerr << "Failed to create GLFW window\n";
            return nullptr;
        }

        glfwMakeContextCurrent(glfw_window);
        
        auto glew_status = glewInit();
        if (glew_status != GLEW_OK)
        {
            std::cerr << "Problem with GLEW: " << glewGetErrorString(glew_status) << "\n";
            return nullptr;
        }

        WindowImpl* p = new WindowImpl(glfw_window);
        return p;
    }

    void window_destroy(WindowImpl* window)
    {
        if (window->glfw_window != nullptr)
            glfwDestroyWindow(window->glfw_window);

        glfwTerminate();
        delete window;
    }

    bool window_should_close(WindowImpl* window)
    {
        return glfwWindowShouldClose(window->glfw_window);
    }
}