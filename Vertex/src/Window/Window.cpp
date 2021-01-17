#include "Window.h"

// Include all implementations of Window
#include "Window/GLFW/GLFWWindow.h"

namespace Vertex
{
    Window* Window::Create(const WindowProperties properties)
    {
        return new GLFWWindow(properties);
    }
}