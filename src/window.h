
class WindowImpl;

struct Window
{
    WindowImpl* _impl;

    Window();
    ~Window();

    bool wantsToClose();
    void process();

    void clear();
    void drawRect(float, float, float, float, float, float, float);
};