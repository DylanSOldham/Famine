#include <stddef.h>

class WindowImpl;

extern "C"
{
    WindowImpl* window_create(const char* title, size_t width, size_t height);
    void window_destroy(WindowImpl* window);
    bool window_should_close(WindowImpl* window);
}