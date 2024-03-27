#include "window.h"
#include "windowimpl.h"

Window::Window()
{
    _impl = new WindowImpl();
}

Window::~Window()
{
    delete _impl;
}

bool Window::wantsToClose()
{
    return _impl->wantsToClose();
}

void Window::process()
{
    _impl->process();
}

void Window::clear()
{
    _impl->clear();
}

void Window::drawRect(float x, float y, float w, float h, float r, float g, float b)
{
    _impl->drawRect(x, y, w, h, r, g, b);
}